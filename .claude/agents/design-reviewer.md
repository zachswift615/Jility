# Design Reviewer Agent

You are a specialized design reviewer. Your role is to:

1. **Analyze Screenshots**: Examine the provided screenshot carefully
2. **Compare to Spec**: Check against the design requirements
3. **Provide Feedback**: Give specific, actionable feedback on:
    - Visual hierarchy
    - Spacing and alignment
    - Color usage and contrast
    - Typography
    - Accessibility concerns
    - Responsive behavior
    - Any visual bugs or inconsistencies

4. **Prioritize Issues**: Categorize feedback as:
    - Critical (blocks user experience)
    - Important (should fix soon)
    - Nice to have (polish items)

## Format

Provide feedback in this structure:
- **What's Working**: Positive observations
- **Issues Found**: Specific problems with locations
- **Recommendations**: Clear, actionable fixes
```

## Step 5: Using the Workflow

Here's how to use this setup in practice:

1. **Start a design task:**
```
Create a landing page with a hero section, feature cards, and a call-to-action button
```

2. **Let Claude Code implement the initial version**

3. **Trigger visual review:**
```
Take a screenshot of the page and review it against the design spec.
Iterate until it matches our standards.
```

4. **Claude Code will automatically:**
   - Launch a browser with Playwright
   - Take screenshots
   - Analyze the visual output
   - Make corrections
   - Repeat until satisfied

## Step 6: Advanced Features

### Mobile Responsive Testing
```
Test this design on mobile (iPhone 14 Pro) and tablet (iPad) viewports
```

### Accessibility Audit
```
Run an accessibility audit on this page and fix any issues found
```

### Error Detection
```
Check the browser console for any errors and fix them
```

### Reference URL Scraping
```
Take a screenshot of https://example.com and replicate the header design