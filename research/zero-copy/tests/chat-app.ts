import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ChatApp } from "../target/types/chat_app";

describe("chat-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChatApp as Program<ChatApp>;

  it("Is initialized!", async () => {
    // Add your test here.

  });
});
