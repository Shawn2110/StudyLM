//! System-prompt template for the chat feature, rendered via Tera.
//! The single template branches on the notebook's `PrepMode` so that the
//! same retrieved sources produce viva-flavored, MCQ-flavored, or
//! long-answer-flavored guidance. Per docs/architecture.md §11 (prompts
//! live as Rust constants, not files on disk).

use std::sync::OnceLock;

use tera::{Context, Tera};

use crate::db::models::{DifficultyFocus, ExamType, Format, PrepMode};
use crate::error::{AppError, AppResult};

const SYSTEM_TEMPLATE: &str = r#"
You are StudyLM, a study assistant helping the student prepare for their **{{ exam_label }}**{% if subject %} on {{ subject }}{% endif %}.

{% if format == "mcq" -%}
Format: MCQ. Keep answers tight, then list likely distractors and why each is wrong.
{% elif format == "short" -%}
Format: short answer. One paragraph, definition-first, then a single example.
{% elif format == "long" -%}
Format: long answer. Structure replies as intro / body / conclusion. Use full sentences.
{% elif format == "oral" -%}
Format: oral / viva. Probe before explaining: ask one clarifying question, then teach in plain language.
{% elif format == "numerical" -%}
Format: numerical. Show the worked solution step-by-step; flag the formula at the top.
{%- else -%}
Format: mixed. Adapt structure to the question shape.
{%- endif %}

{% if difficulty == "conceptual" -%}
Emphasis: build intuition. Prefer analogies and connections to first principles over memorization.
{% elif difficulty == "problem_solving" -%}
Emphasis: applied. Walk through one worked example for any technique you mention.
{% elif difficulty == "memorization" -%}
Emphasis: retention. Surface key formulas, dates, definitions in compact, repeatable form.
{% elif difficulty == "mixed" -%}
Emphasis: balanced — concept first, then a worked example.
{%- endif %}

Cite the sources you draw from with bracketed numeric ids, e.g. **[42]**, matching the `id` attribute of the `<source>` blocks below. Do not invent citations. If the sources do not contain the answer, say so clearly and offer the closest related context.

<sources>
{{ sources }}
</sources>
"#;

const TEMPLATE_NAME: &str = "chat_system";

fn engine() -> &'static Tera {
    static ENGINE: OnceLock<Tera> = OnceLock::new();
    ENGINE.get_or_init(|| {
        let mut t = Tera::default();
        t.add_raw_template(TEMPLATE_NAME, SYSTEM_TEMPLATE)
            .expect("chat system template parses");
        t
    })
}

/// Render the chat system prompt for a notebook's `PrepMode`. `sources` is
/// the already-formatted `<source>` block from `retrieval::format_sources`.
pub fn build_system_prompt(prep: &PrepMode, sources: &str) -> AppResult<String> {
    let mut ctx = Context::new();
    ctx.insert("exam_label", exam_label(prep.exam_type));
    ctx.insert("format", format_key(prep.format));
    ctx.insert(
        "difficulty",
        difficulty_key(prep.difficulty_focus.unwrap_or(DifficultyFocus::Mixed)),
    );
    ctx.insert("subject", &prep.subject);
    ctx.insert("sources", sources);

    engine()
        .render(TEMPLATE_NAME, &ctx)
        .map_err(|e| AppError::Internal(format!("render chat prompt: {e}")))
}

fn exam_label(e: ExamType) -> &'static str {
    match e {
        ExamType::Internal => "internal exam",
        ExamType::Midsem => "mid-semester exam",
        ExamType::Endsem => "end-semester exam",
        ExamType::Viva => "viva voce",
        ExamType::Practical => "practical exam",
        ExamType::Assignment => "assignment",
        ExamType::Competitive => "competitive exam",
        ExamType::Custom => "exam",
    }
}

fn format_key(f: Format) -> &'static str {
    match f {
        Format::Mcq => "mcq",
        Format::Short => "short",
        Format::Long => "long",
        Format::Oral => "oral",
        Format::Numerical => "numerical",
        Format::Mixed => "mixed",
    }
}

fn difficulty_key(d: DifficultyFocus) -> &'static str {
    match d {
        DifficultyFocus::Conceptual => "conceptual",
        DifficultyFocus::ProblemSolving => "problem_solving",
        DifficultyFocus::Memorization => "memorization",
        DifficultyFocus::Mixed => "mixed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prep(format: Format, difficulty: Option<DifficultyFocus>) -> PrepMode {
        PrepMode {
            exam_type: ExamType::Endsem,
            format,
            subject: Some("Thermodynamics".into()),
            duration_minutes: None,
            exam_at: None,
            difficulty_focus: difficulty,
        }
    }

    #[test]
    fn mcq_branch_appears() {
        let p = prep(Format::Mcq, Some(DifficultyFocus::Conceptual));
        let out = build_system_prompt(&p, "<source id=\"1\">x</source>").unwrap();
        assert!(out.contains("Format: MCQ"), "got: {out}");
        assert!(out.contains("build intuition"), "got: {out}");
    }

    #[test]
    fn long_answer_structures_response() {
        let p = prep(Format::Long, Some(DifficultyFocus::ProblemSolving));
        let out = build_system_prompt(&p, "").unwrap();
        assert!(out.contains("intro / body / conclusion"));
        assert!(out.contains("worked example"));
    }

    #[test]
    fn includes_sources_block() {
        let p = prep(Format::Mixed, None);
        let out = build_system_prompt(&p, "<source id=\"7\">body</source>").unwrap();
        assert!(out.contains("<source id=\"7\">"));
    }
}
