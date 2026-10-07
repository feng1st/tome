# bitmap-text Specification

## Purpose
The Latin bitmap-text mechanism: the fonts declared in data and
copied whole from the reference assets, glyph splitting at load, a
zoom-matched font choice, and text rendering as one sprite per glyph.
Floating combat text is the first user; the message system is the
known second.

拉丁位图字体机制：字体在数据中声明、贴图整份复制自参考素材，加载
时切分字形，按缩放挑选字体，文字逐字形以 sprite 渲染。伤害飘字是
第一个用户，消息系统是已知的第二个。

## Requirements

### Requirement: Font Table | 字体表

A display data file SHALL declare the fonts; each entry declares the
font's name, a texture path, a split row height (or the texture's full
height), a baseline, a tracking value, and the character table. The
table SHALL carry the five reference fonts: 1x (full texture height,
baseline 6, tracking -1), 15x (row height 12, baseline 9, tracking
-1), 2x (row height 14, baseline 11, tracking -1), 25x (row height
17, baseline 13, tracking -1), and 3x (row height 22, baseline 17,
tracking -2) — all over the reference's full Latin character table.

一个展示数据文件 SHALL 声明字体；每个条目声明字体名、贴图路径、
切分行高（或贴图全高）、基线、字距与字符表。该表 SHALL 携带五个
参考字体：1x（贴图全高，基线 6，字距 -1）、15x（行高 12，基线
9，字距 -1）、2x（行高 14，基线 11，字距 -1）、25x（行高 17，
基线 13，字距 -1）、3x（行高 22，基线 17，字距 -2）——字符表均
为参考实现的完整拉丁字符表。

#### Scenario: The table parses into five fonts | 字体表解析为五个字体

- **WHEN** the font table is loaded | 加载字体表时
- **THEN** five fonts resolve with the row height, baseline, and tracking values above | 解析出五个字体，行高、基线、字距与上述数值一致

### Requirement: Marker-Column Glyph Splitting | 标记列字形切分

At load, each font's texture SHALL be split within its row height: a
column whose every pixel is fully transparent is a separator, and the
stretches between separators SHALL pair in order with the character
table.

加载时 SHALL 在各字体的行高内切分其贴图：整列像素全透明的列即分
隔列，分隔列之间的区段 SHALL 按序与字符表配对。

#### Scenario: Digit glyphs pair in order | 数字字形按序配对

- **WHEN** a font's texture is split | 切分某字体贴图时
- **THEN** the ten digit characters resolve to ten distinct glyph rectangles in their character-table order | 十个数字字符解析为十个互不相同的字形矩形，顺序与字符表一致

### Requirement: Font Data Validation | 字体数据校验

Startup SHALL fail when a font's texture cannot be read, or when a
font's split yields a glyph count different from its character
table's length; the error SHALL name the file and the font.

某字体贴图无法读取，或某字体切分得到的字形数与其字符表长度不一致
时，SHALL 启动失败；错误信息 SHALL 指明出错文件与字体。

#### Scenario: A glyph-count mismatch fails startup | 字形数不符拒绝启动

- **WHEN** a font's split yields fewer glyphs than its character table declares | 某字体切分得到的字形数少于其字符表声明时
- **THEN** startup fails, the error naming the file and the font | 启动失败，错误信息指明出错文件与该字体

### Requirement: Zoom-Matched Font Selection | 按缩放选字体

Choosing a font for a target glyph height SHALL follow the
reference's thresholds on pt = target × zoom: pt ≥ 19 picks font 3x
(font 25x when 1.5 ≤ pt/19 < 2); pt ≥ 14 picks font 25x (font 2x
when 1.8 ≤ pt/14 < 2); pt ≥ 12 picks font 2x (font 15x when
1.7 ≤ pt/12 < 2); pt ≥ 10 picks font 15x (font 1x when
1.4 ≤ pt/10 < 2); below that, font 1x. The render scale SHALL be the
band's integer ratio divided by the zoom, so the text's screen size
tracks the reference's at any zoom.

按目标字形高度选字体 SHALL 遵循参考实现的阈值：令 pt = 目标 ×
缩放——pt ≥ 19 选 3x 字体（1.5 ≤ pt/19 < 2 时选 25x 字体）；
pt ≥ 14 选 25x 字体（1.8 ≤ pt/14 < 2 时选 2x 字体）；pt ≥ 12 选
2x 字体（1.7 ≤ pt/12 < 2 时选 15x 字体）；pt ≥ 10 选 15x 字体
（1.4 ≤ pt/10 < 2 时选 1x 字体）；再低选 1x 字体。渲染缩放 SHALL
为所在区间的整数倍率除以缩放，使文字在屏幕上的尺寸与参考实现在
任意缩放下一致。

#### Scenario: Floating text at zoom two | 缩放为二时的飘字

- **WHEN** a font is chosen for target glyph height 9 at zoom 2 | 缩放为 2、目标字形高度 9 选字体时
- **THEN** pt is 18, font 25x is picked, and the render scale is one half | pt 为 18，选中 25x 字体，渲染缩放为二分之一

### Requirement: Glyph Sprite Text | 逐字形文字渲染

A text SHALL render as one sprite per glyph, laid side by side with
an advance of glyph width plus tracking, sharing the font's baseline.
Setting a text's transparency SHALL write every glyph sprite's alpha
— there is no inherited transparency.

一段文字 SHALL 渲染为每字形一个 sprite，按字宽加字距的步进并排
摆放，共用该字体基线。设置文字透明度 SHALL 写入每个字形 sprite
的 alpha——没有继承透明度一说。

#### Scenario: Two digits lay side by side | 两个数字并排

- **WHEN** the text "10" renders | 渲染文字"10"时
- **THEN** two glyph sprites appear, the second advanced by the first glyph's width plus the tracking | 出现两个字形 sprite，第二个按第一个字形的字宽加字距步进

#### Scenario: Fading writes every glyph | 淡出写入每个字形

- **WHEN** a text's transparency is set to half | 文字透明度设为一半时
- **THEN** every glyph sprite's alpha reads one half | 每个字形 sprite 的 alpha 都为二分之一
