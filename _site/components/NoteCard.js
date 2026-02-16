class NoteCard extends HTMLElement {
    constructor() {
        super();
        const root = this.attachShadow({mode: "open"});
    }

    static get observedAttributes() {
        return ["title", "tag", "href", "date"];
    }

    connectedCallback() {
        this.render();
    }

    attributeChnagedCallback(name, oldValue, newValue) {
        if (oldValue !== newValue) this.render();
    }

    render() {
        const title = this.getAttribute("title") ?? "Untitled";
        const tag = this.getAttribute("tag") ?? "General";
        const status = this.getAttribute("status") ?? "Active";
        const href = this.getAttribute("href") ?? "";
        const date = this.getAttribute("date") ?? "";

        this.shadowRoot.innerHTML = `
            <style>
                .note-card {
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    width: 100%;
                }

                .left {
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    min-width: 0;
                    gap: 0.15rem;
                }

                .right {
                    display: flex;
                    flex-direction: column;
                    align-items: flex-end;
                    min-width: 0;
                    gap: 0.15rem;
                }

                .tag { font-size: 0.5rem; opacity: 0.9; }
                .status { font-size: 0.5rem; opacity: 0.9; }

                /* keep long titles from pushing the date off-screen */
                .title{ font-weight: 600;  overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
                a.title { color: inherit; text-decoration: none; }

                .date { font-size: 0.9rem; opacity: 0.7; white-space: nowrap;}
            </style>
            <div class="note-card">
                <div class="left">
                    <div class="tag"></div>
                    ${href
                        ? `<a class="title" href="${href}"></a>`
                        : `<div class="title"></div>`
                    }
                </div>
                <div class="right">
                    <div class="status"></div>
                    <div class="date"></div>
                </div>
            </div>
        `;

        this.shadowRoot.querySelector(".title").textContent = title;
        this.shadowRoot.querySelector(".tag").textContent = tag;
        this.shadowRoot.querySelector(".status").textContent = status;
        this.shadowRoot.querySelector(".date").textContent = date;
    }
}

customElements.define("note-card", NoteCard);