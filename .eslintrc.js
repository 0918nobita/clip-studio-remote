module.exports = {
    root: true,
    env: {
        browser: true,
        es6: true,
        node: true,
    },
    extends: ['eslint:recommended', 'plugin:prettier/recommended'],
    plugins: ['simple-import-sort'],
    rules: {
        'simple-import-sort/imports': 'error',
    },
    ignorePatterns: ['client/bundle.js'],
    overrides: [
        {
            files: ['**/*.ts'],
            extends: ['plugin:@typescript-eslint/recommended'],
            plugins: ['@typescript-eslint'],
            parser: '@typescript-eslint/parser',
            parserOptions: {
                sourceType: 'module',
                project: './tsconfig.json',
            },
        },
    ],
};
