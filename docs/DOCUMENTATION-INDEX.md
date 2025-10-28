# Fluix Documentation Index

Complete guide to all Fluix documentation.

## 🎓 For Beginners

**Start here if you're new to Fluix!**

### Step-by-Step Tutorials

1. **[Getting Started](tutorials/01-GETTING-STARTED.md)** ⭐ **START HERE**
   - Installation and setup
   - Your first Fluix app
   - Understanding the basics
   - **Time**: 30 minutes

2. **[Working with Components](tutorials/02-COMPONENTS.md)**
   - All components explained
   - Button, Icon, Select, TextInput, Checkbox
   - Complete examples
   - **Time**: 45 minutes

3. **[Styling and Theming](tutorials/03-STYLING.md)**
   - Component sizing
   - Color system
   - Spacing and layout
   - Design patterns
   - **Time**: 30 minutes

[📖 View All Tutorials →](tutorials/README.md)

## 📖 Reference Documentation

**Quick lookup for specific information**

### Component APIs

- **[Component Reference](COMPONENT-REFERENCE.md)** - Complete API for all components
  - Button, Icon, Select, TextInput, Checkbox
  - Methods, properties, events
  - Code examples

- **[Icon Reference](ICON_REFERENCE.md)** - All 22 icons
  - Icon gallery
  - Usage examples
  - Color and sizing guide
  - How to add custom icons

### Technical Guides

- **[Asset Loading Guide](ASSET_LOADING_GUIDE.md)** - How SVG loading works
  - Understanding AssetSource
  - Embedded vs filesystem assets
  - Troubleshooting

- **[Zed Icon Pattern](ZED_ICON_PATTERN.md)** - Learning from Zed
  - How Zed implements icons
  - GPUI asset system
  - Best practices

### Problem Solving

- **[FAQ](FAQ.md)** - Frequently Asked Questions
  - Common issues and solutions
  - Installation help
  - Component usage tips
  - Performance questions

## 🎯 By Use Case

### "I want to build a form"

1. Read: [Working with Components](tutorials/02-COMPONENTS.md)
2. Check: [Component Reference](COMPONENT-REFERENCE.md) for Select, TextInput, Checkbox
3. See: [examples/select_demo.rs](../examples/select_demo.rs)

### "I want to customize colors/sizes"

1. Read: [Styling and Theming](tutorials/03-STYLING.md)
2. Check: [Component Reference](COMPONENT-REFERENCE.md) for size options
3. See: Theme colors in [COMPONENT-REFERENCE.md](COMPONENT-REFERENCE.md#theme-colors)

### "I want to use icons"

1. Read: [Icon Reference](ICON_REFERENCE.md)
2. Check: All 22 available icons
3. See: [examples/icon_demo.rs](../examples/icon_demo.rs)

### "Icons aren't showing"

1. Read: [FAQ - Why aren't my icons showing?](FAQ.md#why-arent-my-icons-showing)
2. Check: [Asset Loading Guide](ASSET_LOADING_GUIDE.md)
3. Verify: You called `.with_assets(fluix::Assets)`

### "I want to handle events"

1. Read: [Working with Components - Event Handling](tutorials/02-COMPONENTS.md#handling-button-events)
2. Check: [Component Reference](COMPONENT-REFERENCE.md) for event types
3. See: Examples in [examples/](../examples/)

## 📚 Complete Documentation List

### Tutorials (Step-by-Step)
- [Tutorial Index](tutorials/README.md)
- [01 - Getting Started](tutorials/01-GETTING-STARTED.md) ⭐
- [02 - Working with Components](tutorials/02-COMPONENTS.md)
- [03 - Styling and Theming](tutorials/03-STYLING.md)
- [04 - Event Handling](tutorials/04-EVENTS.md) (Coming Soon)
- [05 - Building Forms](tutorials/05-FORMS.md) (Coming Soon)

### Reference Guides
- [Component Reference](COMPONENT-REFERENCE.md) - Complete API
- [Icon Reference](ICON_REFERENCE.md) - All icons
- [Asset Loading Guide](ASSET_LOADING_GUIDE.md) - Technical details
- [FAQ](FAQ.md) - Common questions

### Technical Documentation
- [Icon Solution Summary](ICON_SOLUTION_SUMMARY.md) - How we solved SVG loading
- [Zed Icon Pattern](ZED_ICON_PATTERN.md) - Learning from Zed
- [Icon Implementation](ICON_IMPLEMENTATION.md) - Implementation details
- [Final Summary](FINAL_SUMMARY.md) - Project achievements

### Project Information
- [README](../README.md) - Project overview
- [CHANGELOG](../CHANGELOG.md) - Version history
- [CONTRIBUTING](../CONTRIBUTING.md) - How to contribute
- [ROADMAP](ROADMAP.md) - Future plans

## 🔍 Search by Topic

### Installation
- [Getting Started - Installation](tutorials/01-GETTING-STARTED.md#installation)
- [FAQ - Installation & Setup](FAQ.md#installation--setup)

### Components
- [Component Reference](COMPONENT-REFERENCE.md) - All components
- [Working with Components](tutorials/02-COMPONENTS.md) - Tutorial
- [Examples](../examples/) - Working code

### Icons
- [Icon Reference](ICON_REFERENCE.md) - Complete guide
- [FAQ - Icons](FAQ.md#icons) - Common questions
- [Icon Demo](../examples/icon_demo.rs) - Live example

### Styling
- [Styling and Theming](tutorials/03-STYLING.md) - Complete guide
- [Component Reference - Theme Colors](COMPONENT-REFERENCE.md#theme-colors)
- [FAQ - Styling](FAQ.md#styling)

### Events
- [Working with Components - Events](tutorials/02-COMPONENTS.md#handling-button-events)
- [Component Reference - Events](COMPONENT-REFERENCE.md) - Event types
- [Examples](../examples/) - Event handling examples

### Troubleshooting
- [FAQ - Troubleshooting](FAQ.md#troubleshooting)
- [Asset Loading Guide](ASSET_LOADING_GUIDE.md#common-issues)
- [Getting Started - Common Issues](tutorials/01-GETTING-STARTED.md#common-issues)

## 📊 Documentation Stats

- **Tutorials**: 3 complete, 6 planned
- **Reference Docs**: 4 complete
- **Technical Docs**: 4 complete
- **Examples**: 3 working demos
- **Total Pages**: 15+
- **Code Examples**: 100+

## 🎯 Recommended Learning Paths

### Quick Start (1-2 hours)
Perfect for getting up and running quickly.

1. [Getting Started](tutorials/01-GETTING-STARTED.md) (30 min)
2. [Working with Components](tutorials/02-COMPONENTS.md) (45 min)
3. Build something! (30 min)

### Complete Beginner (4-6 hours)
Comprehensive introduction to Fluix.

1. [Getting Started](tutorials/01-GETTING-STARTED.md)
2. [Working with Components](tutorials/02-COMPONENTS.md)
3. [Styling and Theming](tutorials/03-STYLING.md)
4. Build a complete app!

### Reference User
Already know the basics? Jump to:

- [Component Reference](COMPONENT-REFERENCE.md) - Quick API lookup
- [Icon Reference](ICON_REFERENCE.md) - Icon gallery
- [FAQ](FAQ.md) - Quick answers

## 🤝 Contributing to Documentation

Found an error or want to improve the docs?

1. Check [CONTRIBUTING.md](../CONTRIBUTING.md)
2. Open an issue or PR on [GitHub](https://github.com/lipish/fluix)
3. Help others by answering questions

## 📮 Feedback

We'd love to hear from you!

- **Found a bug in docs?** [Open an issue](https://github.com/lipish/fluix/issues)
- **Have a suggestion?** [Start a discussion](https://github.com/lipish/fluix/discussions)
- **Want to contribute?** [Read the guide](../CONTRIBUTING.md)

---

**Ready to start?** Begin with **[Getting Started →](tutorials/01-GETTING-STARTED.md)**

