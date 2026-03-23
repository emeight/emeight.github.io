class BuildCard extends HTMLElement {
    connectedCallback() {
        const title = this.getAttribute("title") ?? "Draft";
        const href = this.getAttribute("href") ?? "#";
        const image = this.getAttribute("image") ?? "./assets/images/ironman.jpg";
        const tag = this.getAttribute("tag") ?? "Testing";
        const background = this.getAttribute("background") ?? "#656565";
        const tag_color = this.getAttribute("tag_color") ?? "#ffffff";

        this.innerHTML = `
            <style>
                build-card {
                    --card-padding-x: 16px;
                    --card-padding-y: 12px;

                    --row-spacing: 16px;
                    --tag-drop: 20px;

                    display: block;
                    width: 100%;
                    background: ${background};
                }

                build-card > .card {
                    display: grid;
                    grid-template-rows: auto auto;
                    width: 100%;
                    padding-top: var(--card-padding-y);
                    padding-right: var(--card-padding-x);
                    padding-bottom: calc(var(--card-padding-y) + var(--tag-drop));
                    padding-left: var(--card-padding-x);
                    border: var(--hashmark);
                }

                build-card > .card > .row {
                    display: flex;
                    flex-direction: row;
                    align-items: flex-end;
                    height: 100%;
                    width: 100%;

                    border-bottom: var(--hashmark);
                    padding-bottom: var(--row-spacing);
                }

                build-card > .card > .row > h1 {
                    font-size: 36px;
                    font-weight: 100;
                    line-height: 0.85;
                }

                build-card > .card > .row > a {
                    margin-left: auto;
                    white-space: nowrap;

                    display: inline-flex;
                    align-items: flex-end;

                    line-height: 1;
                }

                build-card > .card > .row > a > p {
                    font-size: 18px;
                }

                build-card > .card > .row > a > img {
                    width: 22px;
                    display: block;
                }

                build-card > .card > .media {
                    position: relative;
                    width: 100%;
                    padding-top: var(--row-spacing);
                }

                build-card > .card > .media > img {
                    display: block;
                    width: 100%;
                    height: auto;
                    border-radius: 2px;
                }

                build-card > .card > .media > h2 {
                    position: absolute;
                    right: 5%;
                    bottom: 0;
                    transform: translateY(var(--tag-drop));
                    
                    font-size: 42px;
                    font-weight: 300;
                    color: ${tag_color};
                    line-height: 1;
                    white-space: nowrap;
                }
            </style>
            <div class="card">
                <div class="row">
                    <h1>${title}</h1>
                    <a href=${href}>
                        <p>Explore</p>
                        <img src="./assets/icons/arrow.svg" alt="Explore">
                    </a>
                </div>
                <div class="media">
                    <img src=${image} alt=${title}>
                    <h2>${tag}</h2>
                </div>
            </div>
        `
    }
}

customElements.define("build-card", BuildCard);