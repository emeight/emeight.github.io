class BuildCard extends HTMLElement {
    connectedCallback() {
        const project = this.getAttribute("project") ?? "Draft";
        const number = this.getAttribute("number") ?? "00";
        const status = this.getAttribute("status") ?? "active";
        const stage = this.getAttribute("stage") ?? "Ideating";
        const version = this.getAttribute("version") ?? "0.0.0";
        const tag = this.getAttribute("tag") ?? "";
        const details = this.getAttribute("details") ?? "I'll tell you exactly what's happening, why I oughta.";
        const image = this.getAttribute("image") ?? "./assets/images/ironman.jpg";
        const href = this.getAttribute("href") ?? "#";
        const headline = this.getAttribute("headline") ?? "What's going on here?";

        this.innerHTML = `
            <style>
                build-card {
                    --light-cell: #fff;
                    --dark-cell: #111;

                    --text-row-height: 50px;
                    --media-row-height: 120px;

                    --explore-col-width: 30px;
                    --main-block-width: 300px;

                    border: var(--hashmark);
                }

                build-card > .container {
                    display: grid;
                    grid-template-columns: 300px 1fr var(--explore-col-width);
                }

                build-card > .container > .grid {
                    display: grid;
                    grid-template-rows: var(--text-row-height) var(--media-row-height) var(--text-row-height);

                    overflow: hidden;

                    border-right: var(--hashmark);
                }

                build-card > .container > .grid > .header {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    border-bottom: var(--hashmark);
                }

                build-card > .container  > .grid > .header > .outer {
                    display: grid;
                    place-items: center;
                    height: 100%;
                    aspect-ratio: 1 / 1;

                    background:
                        repeating-linear-gradient(
                            -45deg,
                            #000000 0,
                            #000000 2px,
                            transparent 2px,
                            transparent 6px
                        );
                }

                build-card > .container > .grid > .header > .outer > .inner {
                    display: grid;
                    place-items: center;
                    height: 80%;
                    aspect-ratio: 1 / 1;
                    border: var(--hashmark);
                    background: #000000;
                    color: #fefefe;
                    font-size: 1.5rem;
                    font-weight: 500;
                    line-height: 1;
                }

                build-card > .container > .grid > .header > *:not(:last-child) {
                    border-right: var(--hashmark);
                }

                build-card > .container > .grid > .content {
                    display: grid;
                    grid-template-columns: calc(var(--text-row-height) / 3) 1fr;
                }

                build-card > .container > .grid > .content > .flair {
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

                build-card > .container > .grid > .content > .media {
                    display: grid;
                    place-items: center;
                    height: 100%;
                    width: 100%;
                    overflow: hidden;
                }

                build-card > .container > .grid > .content > .media > img {
                    width: 100%;
                }

                build-card > .container > .grid > .metadata {
                    display: grid;
                    grid-template-columns: auto 1fr 1fr;

                    border-top: var(--hashmark);
                }

                build-card > .container > .grid > .metadata > *:not(:last-child) {
                    border-right: var(--hashmark);
                }

                build-card > .container > .grid > .metadata > .status {
                    display: grid;
                    grid-template-rows: auto 1fr;
                    height: 100%;
                    aspect-ratio: 1 / 1;
                    overflow: hidden;
                }

                build-card > .container > .grid > .metadata > .status > .indicator {
                    --indicator-radius: 12px;

                    place-self: center;
                    width: var(--indicator-radius);
                    height: var(--indicator-radius);
                    border-radius: 50%;
                    animation: blink 3s steps(2, start) infinite;
                }

                build-card > .container > .grid > .metadata > .status > .indicator[data-state="abandoned"] {
                    background: #d92d20;
                    box-shadow: 0 0 10px #d92d20;
                }

                build-card > .container > .grid > .metadata > .status > .indicator[data-state="paused"] {
                    background: #f5b301;
                    box-shadow: 0 0 10px #f5b301;
                }

                build-card > .container > .grid > .metadata > .status > .indicator[data-state="active"] {
                    background: #12b76a;
                    box-shadow: 0 0 10px #12b76a;
                }

                build-card > .container > .grid > .metadata > .status,
                build-card > .container > .grid > .metadata > .status,
                build-card > .container > a > .row > .title {
                    padding: 4px 8px;
                }

                build-card > .container > .overview {
                    border-right: var(--hashmark);
                }

                build-card > .container > .explore {
                    display: grid;
                    grid-template-rows: 1fr var(--explore-col-width);
                }

                build-card > .container > .explore > .tag {
                    writing-mode: vertical-rl;
                    transform: rotate(180deg);
                }

                build-card > .container > .explore > a {
                    display: grid;
                    place-items: center;

                    height: 100%;
                    width: 100%;

                    border-top: var(--hashmark);
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
            <div class="container">
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
                            <img src=${image} alt=${project}>
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
                <div class="overview">
                    <h6>OVERVIEW</h6>
                    <h4>${headline}</h4>   
                    <p>${details}</p>
                </div>
                <div class="explore">
                    <div class="tag">
                        <h6>TAG</h6>
                        <h4>${tag}</h4>                        
                    </div>
                    <a href="${href}">
                        <img src="./assets/icons/arrow.svg" alt="explore">
                    </a>
                </div>
            </div>
        `
    }
}

customElements.define("build-card", BuildCard);