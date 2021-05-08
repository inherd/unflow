# DSL

Flex basicFlex = flex.direction(.column).paddingLeft(12).grow(1)

## resources


Atomic Design: [https://bradfrost.com/blog/post/atomic-web-design/](https://bradfrost.com/blog/post/atomic-web-design/)

Angular Flex-Layout: [https://github.com/angular/flex-layout](https://github.com/angular/flex-layout)

Swift FlexLayout [https://github.com/layoutBox/FlexLayout](https://github.com/layoutBox/FlexLayout)

## PoC

Layout Prepare

Template ->


 - User experience (and IA) concerns like navigation, flows, and tasks
 - Visual style like color, typography, and iconography
 - Interaction with a particular focus on UI patterns, animation & motion

http://pixelkated.com/core-interaction-1/tag/web-design/

1. Content & Structure – Milestone check-in Oct 4
2. Interaction Design – Milestone check-in Oct 11
3. Visual Design – Milestone check-in Oct 18
4. Publish – Due Oct 25
5. Optional: Make it Awesome-r

[https://uxplanet.org/scaling-design-as-the-company-grows-e99af38f9e8d](https://uxplanet.org/scaling-design-as-the-company-grows-e99af38f9e8d)

 - Core Principles
 - Ethical Touchstones
 - Team Structure
 - Design System
 - Process & Methodology
 - Generative Research
 - Behavioral Analytics
 - Measurable ROI
 - Evangelism & Outreach

## Flow


## Behavior

Basic Event:

 - Click
 - Double Click
 - Hover
 - Change
 - Select

Drag Event:

 - drag
 - dragstart
 - dragend
 - dragover
 - dragenter
 - dragleave
 - drop

Touch Event:

| Gesture | Least restrictive touch-action value |
| --- | --- |
| press | auto |
| tap | auto |
| multitap | manipulation |
| vertical pan/swipe | pan-x |
| horizontal pan/swipe | pan-y |
| rotate | pan-x pan-y |
| pinch | pan-x pan-y | 


## Design System

 - https://airbnb.design/designops-airbnb/
 - https://airbnb.design/building-a-visual-language/
 

## Flex

https://github.com/layoutBox/FlexLayout

```java
Flex basicFlex = flex.direction(.column).paddingLeft(12).grow(1)
```

CSS

| FlexLayout Name | CSS Name | React Native Name |
| --- | --- | --- |
| **`direction`** | `flex-direction` | `flexDirection` |
| **`wrap`** | `flex-wrap` | `flexWrap` |
| **`grow`** | `flex-grow` | `flexGrow` |
| **`shrink`** | `flex-shrink` | `flexShrink` |
| **`basis`** | `flex-basis` | `flexBasis` |
| **`start`** | `flex-start` | `flexStart` |
| **`end`** | `flex-end` | `flexEnd` |

*   FlexLayout default properties are sligthly different from CSS flexbox. This table resume these difference:

| Property | FlexLayout default value | CSS default value | React Native default value |
| --- | --- | --- | --- |
| **`direction`** | column | row | column |
| **`justifyContent`** | start | start | start |
| **`alignItems`** | stretch | stretch | stretch |
| **`alignSelf`** | auto | auto | auto |
| **`alignContent`** | start | stretch | start |
| **`grow`** | 0 | 0 | 0 |
| **`shrink`** | 0 | 1 | 0 |
| **`basis`** | 0 | auto | 0 |
| **`wrap`** | noWrap | nowrap | noWrap |

Pix2 Code https://github.com/tonybeltramelli/pix2code