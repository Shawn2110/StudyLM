//! `nomic-embed-text-v1.5` via Candle, CPU-only. 768-d embeddings, mean-pooled
//! over attention-masked tokens with L2 normalization (nomic convention).
//! Model + tokenizer + config are downloaded from the HuggingFace Hub on
//! first use and cached under `~/.cache/huggingface/hub/`.

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::api::tokio::Api;
use tokenizers::{PaddingParams, Tokenizer};

use crate::error::{AppError, AppResult};

/// HuggingFace repo slug for the nomic embedder used across the app.
const REPO: &str = "nomic-ai/nomic-embed-text-v1.5";

pub struct NomicEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl NomicEmbedder {
    /// Download (if missing) the model files from HF Hub, then load the
    /// embedder into memory. Uses the shared HF cache at
    /// `~/.cache/huggingface/hub/`.
    pub async fn load() -> AppResult<Self> {
        let api = Api::new().map_err(into_app)?;
        let repo = api.model(REPO.to_string());

        let config_path = repo.get("config.json").await.map_err(into_app)?;
        let tokenizer_path = repo.get("tokenizer.json").await.map_err(into_app)?;
        let weights_path = repo.get("model.safetensors").await.map_err(into_app)?;

        let config_str = tokio::fs::read_to_string(&config_path).await?;
        let config: Config = serde_json::from_str(&config_str).map_err(into_app)?;

        let mut tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(into_app)?;
        tokenizer.with_padding(Some(PaddingParams::default()));

        let device = Device::Cpu;
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, &device)
                .map_err(into_app)?
        };
        let model = BertModel::load(vb, &config).map_err(into_app)?;

        tracing::info!(repo = REPO, "nomic embedder ready");
        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    /// Embed a batch of texts. Each text becomes a 768-d `Vec<f32>`,
    /// mean-pooled over attention tokens and L2-normalized.
    pub fn embed(&self, texts: &[&str]) -> AppResult<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let encodings = self
            .tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(into_app)?;

        let mut ids_rows = Vec::with_capacity(encodings.len());
        let mut mask_rows = Vec::with_capacity(encodings.len());
        for enc in &encodings {
            ids_rows.push(Tensor::new(enc.get_ids(), &self.device).map_err(into_app)?);
            mask_rows.push(Tensor::new(enc.get_attention_mask(), &self.device).map_err(into_app)?);
        }
        let token_ids = Tensor::stack(&ids_rows, 0).map_err(into_app)?;
        let attention_mask = Tensor::stack(&mask_rows, 0).map_err(into_app)?;
        let token_type_ids = token_ids.zeros_like().map_err(into_app)?;

        let hidden = self
            .model
            .forward(&token_ids, &token_type_ids, Some(&attention_mask))
            .map_err(into_app)?;

        // Mean-pool, weighted by attention mask.
        let mask = attention_mask
            .to_dtype(DTYPE)
            .map_err(into_app)?
            .unsqueeze(2)
            .map_err(into_app)?
            .broadcast_as(hidden.shape())
            .map_err(into_app)?;
        let summed = (&hidden * &mask).map_err(into_app)?.sum(1).map_err(into_app)?;
        let counts = mask.sum(1).map_err(into_app)?;
        let pooled = summed.broadcast_div(&counts).map_err(into_app)?;

        // L2 normalize
        let norm = pooled
            .sqr()
            .map_err(into_app)?
            .sum_keepdim(1)
            .map_err(into_app)?
            .sqrt()
            .map_err(into_app)?;
        let normalized = pooled.broadcast_div(&norm).map_err(into_app)?;

        normalized
            .to_dtype(candle_core::DType::F32)
            .map_err(into_app)?
            .to_vec2::<f32>()
            .map_err(into_app)
    }
}

fn into_app<E: std::fmt::Display>(e: E) -> AppError {
    AppError::Internal(e.to_string())
}
