# language: en
  Feature: Router
    Scenario: Goto HomePage
      Given LoginPage
      When Click Button.Login
      Then Goto HomePage

    Scenario: Goto BlogPage
      Given When HomePage
      When Click Link.Blog
      Then Goto BlogPage

    Scenario: Goto BlogDetailPage
      Given When BlogPage
      When Click Link.Detail
      Then Goto BlogDetailPage
