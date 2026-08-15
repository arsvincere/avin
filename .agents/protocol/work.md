# Work file

Work file хранится как: `.agents/work/<work name>.md`

Имя файла:

- lowercase;
- глагол + объект;
- без даты;
- без номера;

Work file является живым source of truth текущей работы. Он не является
transcript discussion или implementation journal.

# Requirements

До составления work file должны быть определены:
- `Goal`;
- `Expected behavior`;
- `Scope`;

В более сложных работах могут дополнительно быть обсуждены и согласованы:
- `Problem`;
- `Design`;
- `Out of scope`;
- `Constraints`;
- `Implementation plan`;
- `Verification`.

# Work lifecycle

1. Начало работы - создается work file и служебный коммит:

commit `work: <work name>`
- добавляет только новый work file;
- создаёт rollback point перед implementation;

2. Реализация - промежуточные коммиты:

commit `- <commit message>`
- implementation commits идут с префиксом `-`;
- commit message предлагает Karl;

3. Завершение - проверка pre-commit и служебный коммит:

выполнить: `just pre-commit`
- если что-то еще чинили - создать отдельные `- <commit message>`
- в конце работы ветка должна быть зеленой;

commit `work: complete`
- меняет только `Result` и фиксирует фактический итог;

4. Чистка - служебный коммит:

commit `work: remove`
- отдельным commit удаляет финализированный work file из `.agents/work/`.

# Template

Эти блоки обязательно должны быть включены в work file.
- `Goal`;
- `Expected behavior`;
- `Scope`;
- `Result`;

По необходимости фиксируются дополнительные блоки:
- `Problem`;
- `Design`;
- `Out of scope`;
- `Constraints`;
- `Implementation plan`;
- `Verification`.

Если в дополнительный блок нечего записать — исключить его.

Web-Karl показывает Alex полный заполненный draft. После согласования и прямой команды Alex итоговый work file записывается в репо.

```markdown
# <Work name>

## Goal

- Какой конкретный конечный результат должен появиться.

## Expected behavior

- Как система должна вести себя после выполнения работы.

## Scope

- Что входит в работу.

## Problem

- Что сейчас отсутствует, работает неправильно или требует изменения.

## Design

- modules и responsibilities;
- значимые public classes и functions;
- высокоуровневый user-facing public API и его skeletons;
- contracts, dependencies и invariants;
- ownership data и state;
- data flow;
- значимые architecture boundaries.

Web-Karl не задаёт:

- private classes;
- private methods;
- private functions;
- private members;
- private helpers;
- internal decomposition.

## Out of scope

- Что явно не входит в работу.

## Constraints

- Утверждённые ограничения и запрещённые отклонения.

## Implementation plan

- в каком порядке создавать и проверять modules и значимые public constructs.
- dependencies между stages;
- bottom-up или top-down implementation;
- утверждённые temporary stubs;
- direct tests отдельных modules;
- подключение consumers;
- сборку и проверку итоговой vertical slice.

## Verification

- observable behavior;
- обязательные cases;
- contracts и architecture boundaries, которыми подтверждается результат;

## Result

При создании work file этот раздел остается пустым.

После завершения работы кратко фиксируется фактический итог.
```
