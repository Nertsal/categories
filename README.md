# Category theory diagram

Try it out [here](https://nertsal.github.io/categories/).

# Interface

The window is split into three areas (left to right): rules, facts, and goal.

## Rules

Each rule is built from a sequence of constraints, which can be: 'for all' and 'exists'. For example, the identity rule can be represented by the following sentence: For all objects there exists a morphism coming from that object back to itself.

When applying a rule, possible input selections are highlighted.

The rules are color coded the following way:
 - Blue - input; has to be selected by the user
 - Red - output
 - Purple - inferred from context
 - Cyan - forall
 - Green - exists
 
## Facts

Facts can be used together with the rules to infer new facts. To apply the rule, first click on an empty space inside the rule, then select the objects/morphisms that the rule expects (possible options are highlighted).

## Goal

Rules can also be used on the goal to constraint it. For example, to prove that there exists a morphism from A to B, it is sufficient to prove that there exist some object X together with two morphisms A->X, X->B.

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
   