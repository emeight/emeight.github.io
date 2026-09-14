// Client-side pagination: slices a card list into pages

(function () {
    const PAGE_SIZE = 10;
    const PARAM = 'page';

    // flip to true for animation
    const SMOOTH_SCROLL = false;

    document.querySelectorAll('[data-paginate]').forEach(setup);

    function setup(list) {
        const items = Array.from(list.children);
        const total = Math.ceil(items.length / PAGE_SIZE);
        const navs = [list.previousElementSibling, list.nextElementSibling]
            .filter(el => el && el.classList.contains('pagination'));

        if (!navs.length) return;

        // makes the list a focus target without putting it in the tab order
        list.setAttribute('tabindex', '-1');

        const status = createStatus(list);
        const links = createLinks(navs, total, goTo);

        let current = 0;
        let announce = false;

        show(pageFromUrl());
        announce = true;

        window.addEventListener('popstate', () => {
            const page = pageFromUrl();
            if (page === current) return;
            show(page);
            reveal();
        });

        function pageFromUrl() {
            const n = parseInt(new URLSearchParams(location.search).get(PARAM), 10);
            return n >= 1 && n <= total ? n : 1;
        }

        function goTo(page) {
            if (page === current) return;
            const url = new URL(location.href);
            url.searchParams.set(PARAM, page);
            history.pushState(null, '', url);
            show(page);
            reveal();
        }

        function show(page) {
            const start = (page - 1) * PAGE_SIZE;
            const end = start + PAGE_SIZE;

            items.forEach((el, i) => {
                el.hidden = i < start || i >= end;
            });

            links.forEach(a => {
                if (Number(a.dataset.page) === page) {
                    a.setAttribute('aria-current', 'page');
                } else {
                    a.removeAttribute('aria-current');
                }
            });

            current = page;

            // skipped on first paint so a deep link doesn't announce on load
            if (announce) status.textContent = `Page ${page} of ${total}`;
        }

        // scroll only when the top of the list isn't comfortably in view
        function reveal() {
            const top = list.getBoundingClientRect().top;
            if (top >= 0 && top <= window.innerHeight * 0.5) return;

            const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
            window.scrollTo({
                top: window.scrollY + top,
                behavior: SMOOTH_SCROLL && !reduced ? 'smooth' : 'auto'
            });

            // Focus follows the scroll so Tab continues from the new content
            // instead of restarting at the top of the document.
            list.focus({ preventScroll: true });
        }
    }

    function createLinks(navs, total, onSelect) {
        const links = [];

        navs.forEach(nav => {
            nav.innerHTML = '';

            for (let p = 1; p <= total; p++) {
                const a = document.createElement('a');
                a.href = `?${PARAM}=${p}`;
                a.textContent = p;
                a.dataset.page = p;

                a.addEventListener('click', (e) => {
                    // let modified clicks open a real new tab
                    if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
                    e.preventDefault();
                    onSelect(p);
                });

                nav.appendChild(a);
                links.push(a);
            }
        });

        return links;
    }

    function createStatus(list) {
        const el = document.createElement('p');
        el.setAttribute('role', 'status');
        el.setAttribute('aria-live', 'polite');
        el.style.cssText = 'position:absolute;width:1px;height:1px;margin:-1px;' +
            'overflow:hidden;clip-path:inset(50%);white-space:nowrap;';
        list.after(el);
        return el;
    }
})();