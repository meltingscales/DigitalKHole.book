[meta: to help me write the backend as a rust SPA.]
working on a book, I was originally creating this with adobe indesign, and laying out a neat template that looks like `PAGE_TANKA_TEMPLATE.csv` with `content/template.meta.yml` as another guide for data.

I want to make a SPA that lets people read my book.

not sure on stack, but keep it simple and minimal. zero borders, zero bevel, just...
templeos vibe: code and images and data and content. all as one. templeos was/is very beautiful. and wonderfully gauche.

when a template renders, we show, top to bottom, on a single page:

1. tanka // 57757 // <tanka_name>
2. album_qr_code: this should get autogen'd from the album URL
3. album_art: this should get pulled (+cached) from the album URL on bandcamp.
4. recommended_pairing: from yaml.
5. the tanka itself, rendered in big font in the middle. flavor text colors optionally.
6. my commentary on the tanka.
