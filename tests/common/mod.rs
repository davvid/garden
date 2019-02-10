extern crate garden;


pub fn from_yaml_string(string: &String) -> garden::model::Configuration {
    let mut config = garden::model::Configuration::new();
    let file_format = garden::config::FileFormat::YAML;
    garden::config::parse(string, file_format, false, &mut config);

    return config;
}


pub fn from_json_string(string: &String) -> garden::model::Configuration {
    let mut config = garden::model::Configuration::new();
    let file_format = garden::config::FileFormat::JSON;
    garden::config::parse(string, file_format, false, &mut config);

    return config;
}


pub fn garden_config() -> garden::model::Configuration {
    let string = r#"

    templates:
        makefile:
            commands:
                install: make -j prefix=${prefix} install
                test: make test
        python:
            environment:
                PYTHONPATH: ${TREE_PATH}
    trees:
        git:
            url: https://github.com/git/git
            templates: makefile
            variables:
                prefix: ~/.local
            gitconfig:
                user.name: A U Thor
                user.email: author@example.com
        cola:
            url: https://github.com/git-cola/git-cola
            path: git-cola
            templates: [makefile, python]
            variables:
                prefix: ${TREE_PATH}/local
            environment:
                PATH:
                    - ${TREE_PATH}/bin
                    - ${prefix}
                PYTHONPATH: ${TREE_PATH}
            commands:
                test:
                    - git status --short
                    - make test
            remotes:
                davvid: git@github.com:davvid/git-cola.git

    groups:
        cola: [git, qtpy, cola]
        test: [a, b, c]

    gardens:
        cola:
            groups: cola
            variables:
                prefix: ~/src/git-cola/local/git-cola
            environment:
                GIT_COLA_TRACE=: full
                PATH+: ${prefix}
            commands:
                summary:
                    - git branch -vv
                    - git status --short
        git:
            groups: cola
            trees: gitk
            gitconfig:
                user.name: A U Thor
                user.email: author@example.com
    "#.to_string();

    return from_yaml_string(&string);
}