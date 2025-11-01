# GitHub Pages Site

This directory contains the GitHub Pages website for Fluix.

## Structure

- `pages/index.html` - Main page with component documentation (located in root `/pages` directory)
- `pages/styles.css` - Styling for the website
- `pages/script.js` - JavaScript for interactivity
- `docs/index.html` - Redirect page to `/pages/index.html`

## Deployment

GitHub Pages can be configured to serve from the root `/pages` directory:

1. Go to your repository settings
2. Navigate to "Pages" section
3. Select "Deploy from a branch"
4. Choose `main` branch and `/pages` folder
5. Click Save

Alternatively, you can use the `/docs` folder and access the site at `/pages/index.html`.

The site will be available at: `https://<username>.github.io/fluix/` (if configured from `/pages`) or `https://<username>.github.io/fluix/pages/` (if configured from `/docs`)

## Local Development

To preview the site locally:

```bash
# Using Python
python3 -m http.server 8000 --directory docs

# Using Node.js (if you have http-server installed)
npx http-server docs -p 8000

# Then open http://localhost:8000/pages/index.html in your browser
# Or http://localhost:8000/ which will redirect to /pages/index.html
```

## Updating Content

Simply edit the HTML files in this directory and push to GitHub. The site will automatically update.

## Notes

- The site uses vanilla HTML, CSS, and JavaScript (no build step required)
- All styles are in `styles.css`
- All interactivity is in `script.js`
- The design is responsive and works on mobile devices
