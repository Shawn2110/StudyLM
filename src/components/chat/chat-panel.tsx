import { useEffect, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { MessageSquare } from "lucide-react";

import { ChatComposer } from "@/components/chat/chat-composer";
import { MessageBubble } from "@/components/chat/message-bubble";
import {
  createChat,
  listChats,
  listMessages,
  sendChatMessage,
} from "@/lib/commands";
import { onChatStream } from "@/lib/events";
import type { Citation } from "@/lib/events";
import type { Message } from "@/types/bindings";

/*
 * The chat tab inside the notebook detail. Auto-creates a chat on first
 * mount if none exist for the notebook, lists the message history, and
 * subscribes to the `chat-stream` event to render the assistant's reply
 * as it streams in.
 */
export function ChatPanel({ notebookId }: { notebookId: string }) {
  const queryClient = useQueryClient();
  const [selectedModel, setSelectedModel] = useState<string | null>(null);
  const [streamingText, setStreamingText] = useState("");
  const [streamingError, setStreamingError] = useState<string | null>(null);
  const [streamingChatId, setStreamingChatId] = useState<string | null>(null);
  const messagesEnd = useRef<HTMLDivElement | null>(null);

  // 1. Resolve (or create) the chat for this notebook.
  const chats = useQuery({
    queryKey: ["chats", notebookId],
    queryFn: () => listChats(notebookId),
  });
  const ensureChat = useMutation({
    mutationFn: () => createChat(notebookId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["chats", notebookId] });
    },
  });
  const chat = chats.data?.[0];
  useEffect(() => {
    if (chats.data && chats.data.length === 0 && !ensureChat.isPending) {
      ensureChat.mutate();
    }
  }, [chats.data, ensureChat]);

  // 2. Load this chat's persisted messages.
  const messagesQuery = useQuery({
    queryKey: ["messages", chat?.id],
    queryFn: () => listMessages(chat!.id),
    enabled: !!chat,
  });

  // 3. Subscribe to streaming events.
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let active = true;
    onChatStream((p) => {
      if (chat && p.chat_id !== chat.id) return;
      if (p.kind === "delta") {
        setStreamingChatId(p.chat_id);
        setStreamingText((prev) => prev + p.text);
      } else if (p.kind === "done") {
        setStreamingText("");
        setStreamingChatId(null);
        setStreamingError(null);
        queryClient.invalidateQueries({ queryKey: ["messages", p.chat_id] });
      } else if (p.kind === "error") {
        setStreamingError(p.message);
        setStreamingText("");
        setStreamingChatId(null);
      }
    }).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, [chat, queryClient]);

  // 4. Send.
  const send = useMutation({
    mutationFn: ({ text, modelId }: { text: string; modelId: string }) => {
      if (!chat) throw new Error("no chat");
      return sendChatMessage(chat.id, text, modelId);
    },
    onMutate: () => {
      setStreamingError(null);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["messages", chat?.id] });
    },
    onError: (err) => {
      setStreamingError(
        String((err as { message?: string })?.message ?? err),
      );
    },
  });

  // 5. Auto-scroll to bottom on new messages or stream chunks.
  useEffect(() => {
    messagesEnd.current?.scrollIntoView({ behavior: "smooth" });
  }, [messagesQuery.data, streamingText]);

  function handleSend(text: string) {
    if (!selectedModel) return;
    send.mutate({ text, modelId: selectedModel });
  }

  const messages = messagesQuery.data ?? [];
  const showWelcome = messages.length === 0 && streamingText.length === 0;
  const streamingMsg: Pick<Message, "role" | "content" | "citations_json"> | null =
    streamingChatId === chat?.id && streamingText
      ? {
          role: "assistant",
          content: streamingText,
          citations_json: null,
        }
      : null;

  return (
    <section className="flex h-full flex-col gap-4">
      <div className="flex-1 overflow-y-auto">
        {showWelcome && <WelcomeState />}
        <ul className="mx-auto flex max-w-2xl flex-col gap-6 py-4">
          {messages.map((m) => (
            <li key={m.id}>
              <MessageBubble
                message={m}
                onCitationClick={handleCitationClick}
              />
            </li>
          ))}
          {streamingMsg && (
            <li>
              <MessageBubble message={streamingMsg} streaming />
            </li>
          )}
          {streamingError && (
            <li className="rounded-lg border border-danger/40 bg-danger/10 p-3 text-sm text-danger">
              {streamingError}
            </li>
          )}
          <div ref={messagesEnd} />
        </ul>
      </div>
      <div className="mx-auto w-full max-w-2xl">
        <ChatComposer
          selectedModel={selectedModel}
          onSelectedModelChange={setSelectedModel}
          onSend={handleSend}
          busy={send.isPending || streamingText.length > 0}
        />
      </div>
    </section>
  );
}

function handleCitationClick(c: Citation) {
  // PDF jump-to-page lands in P3.7. For now just focus the matching source
  // card via element id; if it isn't on screen, nothing happens.
  const el = document.getElementById(`source-${c.document_id}`);
  el?.scrollIntoView({ behavior: "smooth", block: "center" });
}

function WelcomeState() {
  return (
    <div className="mx-auto mt-12 flex max-w-md flex-col items-center text-center">
      <MessageSquare
        className="h-8 w-8 text-muted-foreground"
        strokeWidth={1.5}
      />
      <p className="mt-3 text-sm leading-relaxed text-muted-foreground">
        Ask anything about this notebook. Answers cite the source page they
        come from.
      </p>
    </div>
  );
}
