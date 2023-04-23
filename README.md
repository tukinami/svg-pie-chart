# svg-pie-chart

[GitHub repository](https://github.com/tukinami/svg-pie-chart)

## これは何?

円グラフを作成し、[`svg`](https://github.com/bodoni/svg)の`Document`形式のデータで返す関数のライブラリです。Rust製。

## 使い方

``` rust
use svg_pie_chart::create_pie_chart;

// (label, ratio, color)
let case = [
    ("Red", 0.5, "#fe5555"),
    ("Green", 0.10, "#55fe55"),
    ("Blue", 0.25, "#3366fe"),
    ("Other", 0.15, "#999"),
];

let pie_chart = create_pie_chart(
    100,         // width
    100,         // height
    40,          // radius of circle
    (0, 0, 0),   // color of label
    10,          // size of label
    20,          // radius of label's position
    &case        // statuses of pies
);

assert!(pie_chart.is_ok());
```

## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [svg](https://github.com/bodoni/svg) / 
    Adam Bryant,
    Felix Schütt,
    GeoffreyY,
    Gijs Burghoorn,
    Ivan Ukhov,
    Mike Wilkerson,
    Nathan Hüsken,
    Nathaniel Cook,
    Nicolas Silva,
    Nor Khasyatillah,
    OCTronics,
    Patrick Chieppe,
    Will Nelson,
    Xander Rudelis,
    e-matteson,
    kmkzt,

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
