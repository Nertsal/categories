# Category theory diagram

Try it out [here](https://nertsal.github.io/categories/).

The window is split into three areas (left to right): rules, facts, and goal.

# Rules

Each rule is built from a sequence of constraints, which can be: 'for all' and 'exists'. For example, the identity rule can be represented by the following sentence: For all objects there exists a morphism coming from that object back to itself.

When applying a rule, possible input selections are highlighted.

The rules are color coded the following way:
 - Blue - input; has to be selected by the user
 - Red - output
 - Purple - inferred from context
 - Cyan - forall
 - Green - exists
 
# Facts

Facts can be used together with the rules to infer new facts. To apply the rule, first click on an empty space inside the rule, then select the objects/morphisms that the rule expects (possible options are highlighted).

# Goal

Rules can also be used on the goal to constraint it. For example, to prove that there exists a morphism from A to B, it is sufficient to prove that there exist some object X together with two morphisms A->X, X->B.

# Controls
 - Move and select objects and morphisms with left click
 - Move camera with right click
 - Ctrl + scroll - zoom camera in/out
 - Left click on an empty space on a rule to select it
