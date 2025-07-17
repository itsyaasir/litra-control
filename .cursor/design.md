# Litra Control App - Design Specification

## Overview
A modern, intuitive desktop application for controlling Logitech Litra devices with a beautiful, responsive interface built with Tauri, Vue 3, TypeScript, Tailwind CSS, and Shadcn/ui components.

## Design Principles
- **Clean & Modern**: Minimalist design with focus on usability
- **Intuitive Controls**: Clear, accessible interface elements
- **Real-time Feedback**: Immediate visual feedback for all actions
- **Responsive**: Works seamlessly across different window sizes
- **Accessibility**: WCAG 2.1 compliant with proper contrast and keyboard navigation

## Visual Design

### Color Palette
- **Primary**: Blue (#3B82F6) - for primary actions and highlights
- **Secondary**: Slate (#64748B) - for secondary elements
- **Success**: Green (#10B981) - for positive states (device on)
- **Warning**: Amber (#F59E0B) - for attention states
- **Error**: Red (#EF4444) - for error states
- **Background**: White (#FFFFFF) / Dark (#0F172A) - adaptive theme
- **Surface**: Gray-50 (#F9FAFB) / Gray-800 (#1F2937) - cards/panels

### Typography
- **Font Family**: Inter (system fallback: -apple-system, BlinkMacSystemFont)
- **Headings**: Font weights 600-700, appropriate sizing hierarchy
- **Body**: Font weight 400, 14-16px for readability
- **Monospace**: JetBrains Mono for technical values (lumens, kelvin)

### Layout Structure

#### Main Window
- **Size**: 800x600px (default), minimum 600x400px
- **Layout**: Single-page application with sidebar navigation
- **Header**: App title, settings, device count indicator
- **Main Content**: Device cards grid/list
- **Footer**: Status bar with connection info

#### Device Card Design
```
┌─────────────────────────────────────┐
│  [Icon] Device Name        [Status] │
│  Serial: ABC123                     │
│  ┌─────────────────────────────────┐ │
│  │ Power: [●] ON                   │ │
│  │ Brightness: [====■────] 65%     │ │
│  │ Temperature: [===■──] 4500K     │ │
│  └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

## Component Specifications

### Device Card Component
- **Status Indicator**: Colored dot (green=on, gray=off, red=error)
- **Power Toggle**: Large, accessible switch component
- **Brightness Slider**: Range 20-400 lumens with percentage display
- **Temperature Slider**: Range 2700K-6500K with color preview
- **Device Info**: Model name, serial number, connection status

### Control Elements
- **Sliders**: Custom styled with Tailwind, smooth animations
- **Buttons**: Rounded, with hover/focus states, loading indicators
- **Toggle Switch**: iOS-style switch with smooth transitions
- **Status Icons**: Phosphor Icons or Heroicons for consistency

### Navigation
- **Sidebar**: Collapsible, device list with quick actions
- **Tabs**: Device overview, settings, help
- **Breadcrumbs**: For multi-level navigation (if needed)

## Interaction Design

### Device Discovery
- Auto-refresh device list every 5 seconds
- Visual indicators for new/disconnected devices
- Skeleton loading states during discovery

### Control Feedback
- Immediate visual feedback on control changes
- Loading states for operations in progress
- Success/error toast notifications
- Smooth animations for state transitions

### Error Handling
- Clear error messages with suggested actions
- Retry mechanisms for failed operations
- Graceful degradation for unsupported features

## Responsive Design

### Window Sizes
- **Large (>800px)**: Grid layout with multiple device cards
- **Medium (600-800px)**: Single column layout
- **Small (400-600px)**: Compact card design with stacked controls

### Dark Mode Support
- System preference detection
- Manual toggle in settings
- Appropriate color adjustments for all components
- Smooth theme transitions

## Accessibility Features
- High contrast mode support
- Keyboard navigation for all controls
- Screen reader compatibility
- Focus indicators and proper ARIA labels
- Reduced motion preferences

## Animation & Transitions
- Smooth slider movements (300ms ease-out)
- Power state transitions (200ms ease-in-out)
- Card hover effects (150ms ease-in-out)
- Loading spinners and progress indicators
- Micro-interactions for better UX

## Technical Implementation
- **Framework**: Vue 3 with Composition API
- **Styling**: Tailwind CSS with custom components
- **Components**: Shadcn/ui adapted for Vue
- **Icons**: Phosphor Icons or Heroicons
- **State Management**: Pinia for global state
- **Animations**: Vue Transition API + Tailwind animations
