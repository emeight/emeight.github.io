class NoteCard extends HTMLElement {
    connectedCallback() {
        const title = this.getAttribute("title") ?? "Attempting to Write Something Sensical";
        const date = this.getAttribute("date") ?? "Feb. 16th, 2002";
        const tag = this.getAttribute("tag") ?? "Thoughts";
        const href = this.getAttribute("href") ?? "#";
        const number = this.getAttribute("number") ?? "00";
        const cells = this.parseCells(this.getAttribute("cells") ?? "[5]");

        this.innerHTML = `
            <style>
                note-card {
                    margin-top: 0;

                    --card-height: 80px;
                    --cell-color: #000000;
                    --number-color: #ffffff;
                }

                note-card > a {
                    display: block;
                    width: 100%;
                    border: var(--hashmark);
                }

                note-card > a > .row {
                    display: grid;
                    grid-template-columns: var(--card-height) auto 1fr;
                    width: 100%;
                    height: var(--card-height);
                    align-items: stretch;
                }

                note-card > a > .row > * {
                    height: 100%;
                    width: 100%;
                    min-width: 0;
                    min-height: 0;
                }

                note-card > a > .row > .grid {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    grid-template-rows: repeat(3, 1fr);
                    width: 100%;
                    height: 100%;
                    aspect-ratio: 1 / 1;
                    place-items: stretch;
                }

                note-card > a > .row > .grid > * {
                    display: grid;
                    place-items: center;
                    width: 100%;
                    height: 100%;
                    min-width: 0;
                    min-height: 0;
                }

                note-card > a > .row > *:not(:last-child) {
                    border-right: var(--hashmark);
                }

                note-card > a > .row > .grid > .filled {
                    background-color: var(--cell-color);
                }
                
                note-card > a > .row > .grid > .number {
                    color: var(--number-color);
                    font-weight: 700;
                }

                note-card > a > .row > .metadata {
                    display: grid;
                    grid-template-rows: repeat(2, 1fr);
                    height: 100%;
                }

                note-card > a > .row > .metadata > .tag,
                note-card > a > .row > .metadata > .date,
                note-card > a > .row > .title {
                    padding: 4px 8px;
                }

                note-card > a > .row > .metadata > .tag {
                    border-bottom: var(--hashmark);
                }
                
            </style>
            <a href="${href}">
                <div class="row">
                    <div class="grid">
                        ${Array.from({ length: 9 }, (_, i) => {
                            const n = i + 1;
                            return `<div class="${cells.includes(n) ? "filled" : ""} ${n === 5 ? "number" : ""}">${n === 5 ? number : ""}</div>`;
                        }).join("")}
                    </div>
                    <div class="metadata">
                        <div class="tag">
                            <h6>TAG</h6>
                            <h4>${tag}</h4>
                        </div>
                        <div class="date">
                            <h6>DATE</h6>
                            <h4>${date}</h4>
                        </div>
                    </div>
                    <div class="title">
                        <h6>TITLE</h6>
                        <h1>${title}</h1>
                    </div>
                </div>
            </a>
        `;
    }
    parseCells(value) {
        try {
            const cells = JSON.parse(value);
            return Array.isArray(cells) && cells.every(n => Number.isInteger(n) && n >= 1 && n <= 9)
            ? [...new Set(cells)]
            : [5];
        } catch {
            return [5];
        }
    }
}

customElements.define("note-card", NoteCard);