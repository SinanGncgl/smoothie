# app/layout.tsx - Root Layout Component

## Overview
The root layout component for the Smoothie Next.js application. This component wraps all pages and provides the basic HTML structure, metadata, and global styling.

## Key Responsibilities
- Define HTML document structure (`<html>`, `<body>`)
- Set page metadata (title, description, icons)
- Load global CSS styles
- Include analytics tracking
- Provide consistent font loading (Geist font family)

## Metadata Configuration

### Page Title and Description
```typescript
export const metadata: Metadata = {
  title: "Smoothie - Workspace Management",
  description: "Automatically manage and restore your preferred screen and tab arrangements across multiple monitors.",
}
```

### Icon Configuration
Supports multiple icon formats and color schemes:
- **Light Mode**: `/icon-light-32x32.png`
- **Dark Mode**: `/icon-dark-32x32.png`
- **SVG Fallback**: `/icon.svg`
- **Apple Touch Icon**: `/apple-icon.png`

## Font Loading
Imports Google Fonts (currently unused but prepared):
```typescript
const _geist = Geist({ subsets: ["latin"] })
const _geistMono = Geist_Mono({ subsets: ["latin"] })
```

## Component Structure
```tsx
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body className={`font-sans antialiased`}>
        {children}
        <Analytics />
      </body>
    </html>
  )
}
```

## CSS Classes Applied
- `font-sans`: Applies sans-serif font family
- `antialiased`: Enables font smoothing for better text rendering

## Analytics Integration
Includes Vercel Analytics for tracking:
```tsx
import { Analytics } from "@vercel/analytics/next"
// ...
<Analytics />
```

## Global Styles
Imports `./globals.css` which includes:
- Tailwind CSS directives
- Custom CSS variables
- Global component styles
- Theme-related styles

## Language and Accessibility
- Sets `lang="en"` for proper screen reader support
- Provides semantic HTML structure

## Performance Considerations
- Fonts are loaded but not applied (marked with `_` prefix)
- Analytics loads asynchronously
- Minimal inline styles for fast rendering

## Browser Support
Compatible with all modern browsers supported by Next.js 16.

## Development Notes
- Uses TypeScript for type safety
- Follows Next.js App Router conventions
- Prepared for internationalization (lang attribute)
- Extensible for additional meta tags or scripts</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/frontend/layout.md