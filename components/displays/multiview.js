class Multiview extends HTMLElement {
    connectedCallback() {
        const title = this.getAttribute("title") ?? "Draft";
        const number = this.getAttribute("number") ?? "000";
        const tag = this.getAttribute("tag") ?? "Prototype";
        const author = this.getAttribute("author") ?? "Terry Emeigh";
        const date = this.getAttribute("date") ?? "02/16/2002";
        const details = this.getAttribute("details") ?? "This is probably super useful.";
        const version = this.getAttribute("version") ?? "v0.0.0";
        const make = this.getAttribute("make") ?? "MK 0";
        const status = this.getAttribute("status") ?? "Prototyping";
        const variant = this.getAttribute("variant") ?? "Base";
        const top = this.getAttribute("top") ?? "./assets/views/default/top.svg";
        const front = this.getAttribute("front") ?? "./assets/views/default/front.svg";
        const isometric = this.getAttribute("isometric") ?? "./assets/views/default/iso.svg";
        const href = this.getAttribute("href") ?? "./builds.html";

        this.innerHTML = `
            <style>
                .multiview {
                    display: grid;
                    grid-template-columns: 1fr auto;
                    width: 100%;
                    height: clamp(300px, 75vh, 900px);
                    border: var(--hashmark);
                    align-items: stretch;
                    overflow: hidden;
                }

                .multiview > .grid {
                    display: grid;
                    grid-template-columns: repeat(2, minmax(0, 1fr));
                    grid-template-rows: repeat(2, minmax(0, 1fr));
                    height: 100%;
                    min-height: 0;
                    width: 100%;
                }

                .multiview > .grid > * {
                    min-height: 0;
                    min-width: 0;
                    width: 100%;
                    height: 100%;
                    overflow: hidden;
                }
                .multiview > .grid > .view > img {
                    width: auto;
                    height: 100%;
                    max-width: 100%;
                    object-fit: contain;
                    display: block;
                    margin: auto;
                    padding: 1rem;
                }

                .multiview > .grid > .block {
                    display: grid;
                    grid-auto-rows: auto;
                    width: 100%;
                    height: auto;
                    max-height: 100%;
                    align-self: end;    
                    border-top: var(--hashmark);
                    border-left: var(--hashmark);
                }

                .multiview > .grid > .block > * {
                    display: grid;
                    width: 100%;
                }

                .multiview > .grid > .block > *:not(:last-child) {
                    border-bottom: var(--hashmark);
                }

                .multiview > .grid > .block > * > * {
                    padding: 4px;
                }

                .multiview > .grid > .block > * > *:not(:last-child){
                    border-right: var(--hashmark);
                }

                .multiview > .grid > .block > .label {
                    grid-template-columns: repeat(3, minmax(0, 1fr));
                }

                .multiview > .grid > .block > .origin {
                    grid-template-columns: repeat(2, minmax(0, 1fr));
                }

                .multiview > .grid > .block > .specs {
                    grid-template-columns: repeat(4, minmax(0, 1fr));
                }

                .multiview > .grid > .block > .details {
                    grid-template-columns: 1fr auto;
                }

                .multiview > .grid > .block > .details > .explore {
                    display: grid;
                    place-items: center;

                    height: 100%;
                    aspect-ratio: 1 / 1;
                }

                .multiview > .grid > .block > .details > .explore > a > img {
                    height: 28px;
                }

                .multiview > .grid > .block > * > * > * {
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }

                .multiview > .ruler {
                    --ruler-pad-y: 4px;
                    position: relative;
                    height: 100%;
                    width: 30px;
                    padding-block: var(--ruler-pad-y);
                    border-left: var(--hashmark);
                }

                .multiview >.ruler::before {
                    content: "";
                    position: absolute;
                    top: var(--ruler-pad-y);
                    bottom: var(--ruler-pad-y);
                    left:0 ;
                    right: 0;

                    background-image: 
                        repeating-linear-gradient(to bottom, transparent 0 9px, black 9px 10px),
                        repeating-linear-gradient(to bottom, transparent 0 49px, black 49px 50px),
                        repeating-linear-gradient(to bottom, transparent 0 99px, black 99px 100px);
                    
                    background-size:
                        25% 10px,
                        50% 50px,
                        75% 100px;
                    
                    background-repeat: repeat-y;
                    background-position: right top;
                }
            </style>
            <div class="multiview">
                <div class="grid">
                    <div class="view">
                        <img src="${isometric}" alt="Isometric View">
                    </div>
                    <div class="view">
                        <img src="${top}" alt="Top View">
                    </div>
                    <div class="view">
                        <img src="${front}" alt="Front View">
                    </div>
                    <div class="block">
                        <div class="label">
                            <div class="number">
                                <h6>SERIAL</h6>
                                <h1>${number}</h1>
                            </div>
                            <div class="title">
                                <h6>PART</h6>
                                <h1>${title}</h1>
                            </div>
                            <div class="tag">
                                <h6>CLASS</h6>
                                <h1>${tag}</h1>
                            </div>
                        </div>
                        <div class="origin">
                            <div class="author">
                                <h6>AUTHOR</h6>
                                <h1>${author}</h1>
                            </div>
                            <div class="date">
                                <h6>DATE</h6>
                                <h1>${date}</h1>
                            </div>
                        </div>
                        <div class="specs">
                            <div class="version">
                                <h6>VERSION</h6>
                                <p>${version}</p>
                            </div>
                            <div class="make">
                                <h6>MAKE</h6>
                                <p>${make}</p>
                            </div>
                            <div class="variant">
                                <h6>VARIANT</h6>
                                <p>${variant}</p>
                            </div>
                            <div class="status">
                                <h6>STATUS</h6>
                                <p>${status}</p>
                            </div>
                        </div>
                        <div class="details">
                            <div class="description">
                                <h6>DETAILS</h6>
                                <p>${details}</p>
                            </div>
                            <div class="explore">
                                <a href="${href}">
                                    <img src="./assets/icons/arrow.svg" alt="explore">
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="ruler"></div>
            </div>
        `
    }
}

customElements.define("multi-view", Multiview);