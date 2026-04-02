class Navbar extends HTMLElement {
    connectedCallback() {
        const toggle = this.getAttribute("toggle") ?? null;

        this.innerHTML = `
            <style>
                .row {
                    display: grid;
                    grid-template-columns: auto 1fr auto;
                    height: 30px;
                    overflow: hidden;
                }
                
                .logo {
                    aspect-ratio: 1 / 1;
                    height: 100%;

                    padding: 2px;
                }

                .links {
                    width: 100%;
                    height: 100%;

                    display: flex;
                    align-item: center;
                    justify-content: center;
                }

                .links > a {
                    padding: 0px 8px;
                }
            </style>
            <div class="row">
                <div class="logo">
                    <a href="/">T</a>
                </div>
                <div class=""></div>
                <div class="links">
                    <a href="/notes">Notes</a>
                    <a href="/builds">Builds</a>
                </div>
            </div>
        `
    }
}

customElements.define("nav-bar", Navbar);