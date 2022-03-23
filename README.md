# Category theory diagram

# About

This tool implements some basic Category theory in the form of a diagram with the intention to help to visualize and understand the concepts in Category theory. In the current state it is possible to prove some simple facts, such as AxB≃BxA, Ax1≃A.

An interactable version of the tool is available [here](https://nertsal.github.io/categories/).

# Basics of Category theory

## Definition of a category

A category consists of a collection of objects, and a collection of morphisms. 

Each morphism has a domain and a codomain. For example, a morphism **f** with domain **A** and codomain **B** is denoted as: `f: A -> B`.

Each object **A** has a corresponding identity morphism **idₐ** (sometimes denoted simply **id** or **1**).

![image](https://user-images.githubusercontent.com/12630585/159643722-5566bac3-3449-4aba-bbd5-ae2af57a6d25.png)

Morphisms **f** and **g** can be composed together if the codomain of **f** is the same as the domain of **g** (i.e. `f: A -> B`, `g: B -> C`). Their composition is `g ∘ f: A -> C`. The category is closed under composition.

![image](https://user-images.githubusercontent.com/12630585/159643777-d608d1f1-503a-475f-a636-df1a16aa42f9.png)

Axioms:
  1. Identity: `f ∘ id = f`, `id ∘ f = f`
  2. Associativity: `f ∘ (g ∘ h)` = `(f ∘ g) ∘ h` = `f ∘ g ∘ h`

## Terminal object

A terminal object (denoted as `1`) is such an object that for all objects in the category there exists a unique morphism from that object to the terminal object.

![image](https://user-images.githubusercontent.com/12630585/159642907-06e1b52c-6522-4f71-9559-ba0b78104f34.png)

## Initial object

An initial object (denoted as `0`) is such an object that for all objects in the category there exists a unique morphism from the initial object to that object (the opposite of terminal object).

![image](https://user-images.githubusercontent.com/12630585/159643274-dff12ee1-27b5-4741-a554-aa028ba67544.png)

## Isomorphism

Two object are called isomorphic if and only if there exists a morphism from both objects to the other and the morphisms are inverse, i.e. their composition in any order equals identity.

![image](https://user-images.githubusercontent.com/12630585/159643552-32d7a052-8914-4a58-b03b-f34dbde1957c.png)

## Product

A product **AxB** of two objects **A** and **B** has two morphisms: `π₁: AxB -> A`, `π₂: AxB -> B`
 - and for all objects **C** with morphisms: `f: C -> A`, `g: C -> B`
 - there exists a morphism `m: C -> AxB`
 - such that the triangles commute: `π₁ ∘ m = f`, `π₂ ∘ m = g` and **m** is unique under that constraint (i.e. if there exists another morphism **m'** such that `π₁ ∘ m' = f`, `π₂ ∘ m' = g` then `m = m'`)

![image](https://user-images.githubusercontent.com/12630585/159645099-7cf8fef7-f6e8-4205-88ce-620811c8879e.png)

# User Interface

The window is split into three areas (left to right): rules, facts, and goal.

## Rules

Each rule is built from a sequence of constraints, which can be: **for all** and **exists**. For example, the identity rule can be represented by the following sentence: **For all** objects there **exists** a morphism coming from that object back to itself.

When applying a rule, possible input selections are highlighted.

The rules are color coded in the following way:
 - Blue - input; has to be selected by the user
 - Red - output
 - Purple - inferred from context
 - Cyan - forall
 - Green - exists

![image](https://user-images.githubusercontent.com/12630585/159646355-7d53765a-e9cc-4fc9-a9d8-9e4868f923d1.png)

For example the terminal object rule can be read as:
 - there exists a terminal object **1**
 - such that for all objects **A** <- input
 - there exists a unique (indicated by a dashed line) morphism from **A** to **1** <- output
 
## Facts

Facts can be used together with the rules to infer new facts. To apply a rule, first click on the rule, then select the objects/morphisms that the rule expects (possible options are highlighted).

## Goal

Rules can also be used on the goal to constraint it. For example, to prove that there exists a morphism from A to B, it is sufficient to prove that there exist some object X together with two morphisms A->X, X->B.

For example, the picture below means that the goal is prove that objects **A** and **Ax1** (the product of **A** and the terminal object **1**) are isomorphic)

![image](https://user-images.githubusercontent.com/12630585/159645880-834adb29-8748-4528-8bec-e069f89956b4.png)

## Controls
Both keyboard+mouse and touchscreen are supported. Hopefully, the controls are intuitive, but anyway here is the list of possible actions:
 - Moving objects/morphisms
   - Drag with **LMB**
   - Drag with one finger
 - Moving camera
   - Drag with **RMB**
   - **LCtrl** + drag with **LMB**
   - Drag from an empty place with one finger (not possible in the rule section)
   - Drag with two fingers
 - Zooming the camera
   - **LCtrl** + scroll the mouse wheel
   - Touch with two fingers and control the distance between them
 - Selecting a rule
   - Left click or touch any point inside the rule
 - Selecting an object/morphism
   - Left click or touch the object/morphism
 - Scrolling the rules
   - Scroll the mouse wheel
   - Drag with one finger in the rule area
 - Undo last action
   - **LCtrl** + Z
   - left click or tap the undo button
 - Redo (a.k.a undo last undo)
   - **LCtrl** + **LShift** + Z
   - Left click or tap the redo button
 - Cancel rule selection
   - **Escape**
