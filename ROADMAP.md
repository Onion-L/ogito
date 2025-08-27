# `ogito` Roadmap

This document outlines the future development direction for `ogito`. The goal is to enhance its capabilities as a lightweight and flexible project templating tool.

### Core Command Implementation

- [x] Implement the `list` command to show all saved templates.
- [x] Implement the `remove` command to delete a saved template.
- [x] Implement the `update` command to refresh templates from their source.

### Feature Enhancements

- [x] Support local filesystem paths as a source for templates.
- [ ] Introduce variable substitution in templates (e.g., `{{project_name}}`).
- [ ] Implement a validation system for template structures.
- [ ] Allow running pre/post-generation scripts defined in templates.
- [ ] **Improve `ogito new` command argument structure:** Change `ogito new <template-name> -d <project-name>` to `ogito new <template-name> [<project-name>]`. If `<project-name>` is omitted, prompt the user interactively for the name.
- [ ] **Predefine common template aliases:** Add built-in aliases for popular frameworks (e.g., `react`, `vue`, `node`) that map to official starter templates, simplifying the onboarding process for new users.
- [ ] **Interactive `remove` command:** When `ogito remove` is run without arguments, enter an interactive mode that lists all templates and allows the user to select which ones to delete.

### Developer Experience & Project Health

- [ ] Create a global configuration file (e.g., `~/.ogitorc`) for default settings.
- [x] Enhance documentation and guides on the official website.
- [ ] Refine error handling for network and filesystem operations.
