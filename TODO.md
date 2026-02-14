# TODO

## Features

- [x] DONE! make a request handler for `favicon.ico` so it is a randomly generated pixel art perlin noise "circle" that gets darker near the center - to mimic the Synthetic Reality Well of Souls 'pi' grain of sand dropping circle estimation utility/toy, except this time it's biased towards making a hole icon that's somewhat noisy.

- question: do the dates in the `/` home page actually reflect when the .tanka.yml files were last modified? if not, let's make sure they do.

- add yaml spec `guest_contributor.name` support for 4lung's contributed tankas.

- [ ] **XML color text engine** — Parse `<color:red>text</color>` style markup for flavor text in tankas and commentary. Could support:
  - Named colors: `<color:red>`, `<color:cyan>`
  - Hex colors: `<color:#ff6b6b>`
  - Maybe effects: `<glitch>`, `<blink>`, `<fade>`

- [ ] **Keyboard navigation** — `j`/`k` or arrow keys to move between tankas, `q` to return to index

- [ ] **Search/filter** — Filter tankas by artist, album, or keyword on index page

- [ ] **RSS feed** — Generate feed for new tankas

- [ ] **Dark/light theme toggle** — Some people have eyes that work differently

- [ ] **Mobile layout** — Media row probably needs to stack vertically on small screens

- [ ] **Print stylesheet** — For physical zine output

- [ ] **Audio autoplay option** — Start playing the bandcamp embed on page load (user opt-in)

## Tech debt

- [ ] Consistent file naming (`.tanka.yml` vs `.yml`)

- [ ] Extract shared structs between main.rs and validate.rs (currently duplicated)

- [ ] Add `--release` WASM size optimization (currently ~1.2MB dev, should be ~200KB release)

## Content

- [ ] More tankas
