import { ItemView, Notice } from "obsidian";
import { engineVersion } from "../engine/wasm";

export const VIEW_TYPE_LOPHELIA = "lophelia";

export class LopheliaView extends ItemView {
  getViewType(): string {
    return VIEW_TYPE_LOPHELIA;
  }

  getDisplayText(): string {
    return "Lophelia";
  }

  async onOpen(): Promise<void> {
    this.contentEl.empty();
    this.contentEl.addClass("lophelia-view");
    const container = this.contentEl
      .createDiv({ cls: "empty-state" })
      .createDiv({ cls: "empty-state-container" });

    try {
      container.createDiv({ cls: "lophelia-engine-version" }).setText(`Moteur ${engineVersion()}`);
    } catch (error) {
      container.createDiv({ cls: "lophelia-engine-error" }).setText("Moteur non chargé");
      const reason = error instanceof Error ? error.message : String(error);
      new Notice(`Lophelia : le moteur n'a pas pu être chargé. ${reason}`);
    }
  }
}
