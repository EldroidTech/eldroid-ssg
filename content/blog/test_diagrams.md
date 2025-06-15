---
title: Diagram Support Test
date: 2025-06-15
tags: [test, diagrams, mermaid, graphviz, plantuml]
description: Test and showcase for built-in diagram support in Eldroid SSG markdown.
---

# Diagram Support Test

## Mermaid Example
```mermaid
graph TD
    A[Start] --> B{Is Mermaid enabled?}
    B -- Yes --> C[Render Diagram]
    B -- No --> D[Show Code]
```

## Graphviz Example
```graphviz
digraph G {
  A -> B;
  B -> C;
  C -> A;
}
```

## PlantUML Example
```plantuml
@startuml
Alice -> Bob: Hello
@enduml
```

---

You should see a Mermaid diagram, a Graphviz diagram, and a PlantUML code block above. For PlantUML, use a PlantUML server or pre-rendered images for production.
