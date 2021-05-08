# Layout Engine

## AuotLayout.js


[autolayout.js](https://github.com/IjzerenHein/autolayout.js)

```
//spacing: [10, 20]
'H:|[view1(==view2)]-10-[view2]|',
'V:|[view1,view2]|'
```

More Examples:

```
| features | Examples |
|----------------------------------------|------------------|
|Proportional size                       | (|-[view1(==50%)]) |
|Operators                               | (|-[view1(==view2/2-10)]-[view2]-|) |
|Attributes                              | (V:|[view2(view1.width)]) |
|Z-ordering                              | (Z:|-[view1][view2]) |
|Equal size spacers/centering            | (|~[center(100)]~|) |
|View stacks                             | (V:|[column:[header(50)][content][footer(50)]]|) |
|View ranges (spread operator)           | (H:[view1..8(10)]|) |
|Multiple views                          | (Z:|[background][text1,text2,text3]|) |
|Multiple orientations                   | (HV:|[background]|) |
|Disconnections (right/bottom alignment) | (|[view1(200)]->[view2(100)]|) |
|Negative values (overlapping views)     | (|[view1]-(-10)-[view2]|) |
|Explicit constraint syntax              | (C:view1.centerX(view2.centerX)) |
|Comments                                | ([view1(view1.height/3)]  |
```

[Online Editor Engine](https://github.com/IjzerenHein/visualformat-editor)

## [muuri](https://github.com/haltu/muuri)

