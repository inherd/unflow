# id: TUgT7FxZg
# startDate: 2019-11-22T01:56:41Z
# endDate: 2019-11-22T02:06:48Z
# priority: 
# status: thinking
# author: phodal
# title: image to dsl
# language: zh-CN
@math
Feature:image to dsl

  Scenario: 作为设计师，我想直接获得一份草图生成的 DSL
    Given 设计
    When 当我在设计的时候
    Then 我能将草稿转成 DSL
    Then 这样我能直接将草图转成 SVG
    Then 开发人员可以直接修改代码
