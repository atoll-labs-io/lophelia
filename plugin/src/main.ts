import { Plugin } from "obsidian";
import { LopheliaView, VIEW_TYPE_LOPHELIA } from "./view/lophelia-view";

export default class LopheliaPlugin extends Plugin {
  async onload(): Promise<void> {
    this.registerView(VIEW_TYPE_LOPHELIA, (leaf) => new LopheliaView(leaf));

    this.addCommand({
      id: "open",
      name: "Ouvrir Lophelia",
      callback: () => {
        void this.openView();
      },
    });
  }

  private async openView(): Promise<void> {
    const { workspace } = this.app;
    const existing = workspace.getLeavesOfType(VIEW_TYPE_LOPHELIA)[0];
    if (existing) {
      await workspace.revealLeaf(existing);
      return;
    }
    const leaf = workspace.getLeaf("tab");
    await leaf.setViewState({ type: VIEW_TYPE_LOPHELIA, active: true });
    await workspace.revealLeaf(leaf);
  }
}
