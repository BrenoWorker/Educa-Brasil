# Educa Brasil

Aplicativo de educação para o ensino fundamental (1º ao 9º ano), feito em Rust com [Dioxus](https://dioxuslabs.com). Um único código roda em Android, iOS, Windows, Linux, macOS e na web.

## Recursos

- Aulas curtas de Português, Matemática, Ciências, História e Geografia para cada ano
- Quiz com explicação de cada resposta e alternativas embaralhadas a cada tentativa
- Estrelas, XP e níveis, com progresso salvo no aparelho
- Tema claro e escuro automático

## Conteúdo

O conteúdo fica em `content/fundamental.json`. Cada tema tem `id`, `year`, `subject` (`portugues`, `matematica`, `ciencias`, `historia` ou `geografia`), `title`, `summary`, `lesson`, `example` e `questions`. Para adicionar um tema, basta incluir um novo objeto no arquivo.

## Rodando

```bash
cargo binstall dioxus-cli@0.7.10
dx serve --platform web
dx serve --platform linux
dx serve --platform android
```

## Builds

O GitHub Actions gera a versão web, os instaladores de Linux e Windows (x64 e arm64), o app de macOS (arm64), o APK de Android arm64-v8a com o ícone do app e uma IPA de iOS sem assinatura (para instalar por sideload).

Os ícones de Android ficam em `android/res` e são aplicados sobre o projeto Gradle gerado pelo dx.
