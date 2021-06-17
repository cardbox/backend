# Cardbox API

## Общее

- Я бы предпочтел следовать стандартным HTTP методам (`GET` / `POST` / `PUT` / `PATCH` / `DELETE`)
- Когда происходят ошибки при валидации и проч - я бы указывал это явно в респонсе
    > Чтобы пользователю можно было отобразить алерт с этим, либо же хотя бы как-то с маппить во что-то человеко читабельное, а не просто "Произошла неведомая ошибка. Попробуйте позднее"
- Надо еще подумать, в каком формате отдавать респонс
    > Пока в голове что-то вроде такого крутится
    >
    > `{ data: ..., total: ..., error: ..., ... }`

## Модели

```ts
type User = {
    id: number;
    // В будущем скорее всего будут еще роли
    // Также это будет учитываться в эндпоинтах
    username: string;
    firstName?: string;
    lastName?: string
    bio?: string;
    // Ну эт на будущее прям, возможно даж в MVP не успеем завезти
    // Мб пока вполне хватит URL-string формата
    avatar: File;
    cards: Card[];
    // URL
    github?: string;
    // Либо потом отдельной Work-сущностью
    work: string;
}


type Card = {
    id: number;
    title: string;
    // В будущем можно добавить такое поле
    // Для отображения кастомного овервью-текста в списке (где не оч "обрезать первые n символов")
    // summary?: string;
    content: string;
    // datetime
    createdAt: string;
    // datetime
    updatedAt: string;
    author: User;
    // А потом можно в отдельную сущность после v0
    tags: string[];
}

type File = {
    id: string;
    path: string;
}
```

## Эндпоинты

Скорей всего нужно налепить обычные CRUD'ы на каждую модель, но тут описал те - на которые были тикеты

### Текущий пользователь

#### `GET /api/v1/user/viewer`

Получение текущего пользователя (вьювера)

```ts
// По идее, должно определяться по токену
type Params =  {}
```

```ts
type Response = {
    data: User;
}
```

### Получение списка карточек

#### `GET /api/v1/card`

Общий эндпоинт сразу для нескольких юзкейсов

1. **Карточки из личной коллекции**
    - `GET /api/v1/card?authorId={USER_ID}`;

2. **Карточки по поисковой фразе**
    - `GET /api/v1/card?search={SEARCH_TERM}`;
    - Должен быть поиск по title / content / tags
        - Возможно и по автору - но сомневаюсь

> В будущем скорее всего будет регулироваться еще и "публичность" карточки и доступ самого пользователя к определенным карточкам

```ts
type Params =  {
    // Либо если получится OData-like фильтры - то вообще збс было бы
    authorId?: number;
    // Для реализации поиска
    search?: string;
};
```

```ts
type Response = {
    data: Card[];
    total: number;
}
```

### Получение карточки

#### `GET /api/v1/card/:cardId`

```ts
type Params =  {
    cardId: number
};
```

```ts
type Response = {
    data: Card;
}
```

### Создание карточки

#### `POST /api/v1/card`

```ts
type Params =  {
    cardId: number
} & CardCreateDTO;

// Редактируемые поля - попозже будет расширяться список скорей всего
type CardCreateDTO = {
    title: string;
    content: string;
    tags: string[];
}
// author -> из токена авторизации
// createdAt, updatedAt -> по классике
```

```ts
// В идеале - сразу отдавать обновленную модель, чтобы не дублировать эту логику
type Response = {
    data?: Card;
    // affected: boolean;
    error?: string;
}
```

### Редактирование карточки

#### `PUT /api/v1/card/:cardId`

- Должна быть проверка, что редактируется карточка, которая принадлежит Юзеру (чтобы он не мог редачить чужое, даж через API)

```ts
type Params =  {
    cardId: number
} & CardUpdateDTO;

// Редактируемые поля - попозже будет расширяться список скорей всего
type CardUpdateDTO = {
    title?: string;
    content?: string;
    tags?: string[];
}
// author -> из токена авторизации
// updatedAt -> по классике
```

```ts
// В идеале - сразу отдавать обновленную модель, чтобы не дублировать эту логику
type Response = {
    data?: Card;
    // affected: boolean;
    error?: string;
}
```

### Удаление карточки

#### `DELETE /api/v1/card/:cardId`

- Должна быть проверка, что удаляется карточка, которая принадлежит Юзеру (чтобы он не мог редачить чужое, даж через API)

```ts
type Params = {
    cardId: number
}
```

```ts
type Response = {
    // Или другое подобное булево поле означающее - что карточка действительно удалена и нет ничего запрещающего
    // Есть мнение, что HTTP-кодов возможно не хватит
    // Но мб щас булево поле и оверхедно будет
    affected: boolean;
}
```
