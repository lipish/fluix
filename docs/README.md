# GitHub Pages Site

This directory contains the GitHub Pages website for Fluix.

## Structure

- `index.html` - Redirect page to `pages/index.html`
- `pages/index.html` - Main page with component documentation
- `pages/styles.css` - Styling for the website
- `pages/script.js` - JavaScript for interactivity

## Deployment

GitHub Pages will automatically deploy the contents of this `docs/` directory.

To enable GitHub Pages:

1. Go to your repository settings
2. Navigate to "Pages" section
3. Select "Deploy from a branch"
4. Choose `main` branch and `/docs` folder
5. Click Save

The site will be available at: `https://<username>.github.io/fluix/`

## Local Development

To preview the site locally:

```bash
# Using Python
python3 -m http.server 8000 --directory docs

# Using Node.js (if you have http-server installed)
npx http-server docs -p 8000

# Then open http://localhost:8000/pages/index.html in your browser
# Or http://localhost:8000/ which will redirect to pages/index.html
```

## Updating Content

Simply edit the HTML files in this directory and push to GitHub. The site will automatically update.

## Notes

- The site uses vanilla HTML, CSS, and JavaScript (no build step required)
- All styles are in `styles.css`
- All interactivity is in `script.js`
- The design is responsive and works on mobile devices
