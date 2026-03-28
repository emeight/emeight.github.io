class BuildCard extends HTMLElement {
    connectedCallback() {
        const project = this.getAttribute("project") ?? "Draft";
        const number = this.getAttribute("number") ?? "0";
        const status = this.getAttribute("status") ?? "active";
        const stage = this.getAttribute("stage") ?? "Ideating";
        const version = this.getAttribute("version") ?? "0.0.0";
        const tag = this.getAttribute("tag") ?? "";
        const details = this.getAttribute("details") ?? "";

        this.innerHTML = `
            <style>
                build-card {
                    --light-cell: #fff;
                    --dark-cell: #111;

                    --text-row-height: 50px;
                    --media-row-height: 120px;
                }

                build-card > .grid {
                    display: grid;
                    grid-template-rows: var(--text-row-height) var(--media-row-height) var(--text-row-height);

                    border: var(--hashmark);
                    overflow: hidden;
                }

                build-card > .grid > .header {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    border-bottom: var(--hashmark);
                }

                build-card > .grid > .header > .outer {
                    display: grid;
                    place-items: center;
                    height: 100%;
                    aspect-ratio: 1 / 1;

                    background:
                        repeating-linear-gradient(
                            -45deg,
                            #d9d9d9 0,
                            #d9d9d9 2px,
                            transparent 2px,
                            transparent 6px
                        );
                }

                build-card > .grid > .header > .inner {
                    display: grid;
                    place-items: center;
                    height: 80%;
                    aspect-ratio: 1 / 1;
                    background: black;
                    border: 2px solid #111;
                    font-size: 2rem;
                    font-weight: 700;
                    line-height: 1;
                }

                build-card > .grid > .header > *:not(:last-child) {
                    border-right: var(--hashmark);
                }

                build-card > .grid > .content {
                    display: grid;
                    grid-template-columns: calc(var(--text-row-height) / 3) 1fr;
                }

                build-card > .grid > .content > .flair {
                    height: 100%;
                    width: 100%;
                    border-right: var(--hashmark);
                    background: linear-gradient(
                        to bottom,
                        var(--dark-cell) 0%, var(--dark-cell) 20%,
                        var(--light-cell) 20%, var(--light-cell) 40%,
                        var(--dark-cell) 40%, var(--dark-cell) 60%,
                        var(--light-cell) 60%, var(--light-cell) 80%,
                        var(--dark-cell) 80%, var(--dark-cell) 100%
                    );
                }

                build-card > .grid > .metadata {
                    display: grid;
                    grid-template-columns: auto 1fr 1fr;

                    border-top: var(--hashmark);
                }

                build-card > .grid > .metadata > *:not(:last-child) {
                    border-right: var(--hashmark);
                }

                build-card > .grid > .metadata > .status {
                    display: grid;
                    grid-template-rows: auto 1fr;
                    height: 100%;
                    aspect-ratio: 1 / 1;
                    overflow: hidden;
                }

                build-card > .grid > .metadata > .status > .indicator {
                    --indicator-radius: 12px;

                    place-self: center;
                    width: var(--indicator-radius);
                    height: var(--indicator-radius);
                    border-radius: 50%;
                    animation: blink 3s steps(2, start) infinite;
                }

                build-card > .grid > .metadata > .status > .indicator[data-state="abandoned"] {
                    background: #d92d20;
                    box-shadow: 0 0 10px #d92d20;
                }

                build-card > .grid > .metadata > .status > .indicator[data-state="paused"] {
                    background: #f5b301;
                    box-shadow: 0 0 10px #f5b301;
                }

                build-card > .grid > .metadata > .status > .indicator[data-state="active"] {
                    background: #12b76a;
                    box-shadow: 0 0 10px #12b76a;
                }

                @keyframes blink {
                    0%, 49% {
                        opacity: 1;
                    }
                    50%, 100% {
                        opacity: 0.25;
                    }
                }
            </style>
            <div class="grid">
                <div class="header">
                    <div class="outer">
                        <div class="inner">${number}</div>
                    </div>
                    <div class="project">
                            <h6>PROJECT</h6>
                            <h4>${project}</h4>
                    </div>
                </div>
                <div class="content">
                    <div class="flair"></div>
                    <div class="media">
                        <img src="" alt="">
                        <div class="tag">${tag}</div>
                    </div>
                </div>
                <div class="metadata">
                    <div class="status">
                        <h6>STATUS</h6>
                        <div class="indicator" data-state=${status}></div>
                    </div>
                    <div class="stage">
                        <h6>STAGE</h6>
                        <h4>${stage}</h4>
                    </div>
                    <div class="version">
                        <h6>VERSION</h6>
                        <h4>${version}</h4>
                    </div>
                </div>
            </div>
        `
    }
}

customElements.define("build-card", BuildCard);