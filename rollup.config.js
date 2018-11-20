import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/green-thumb.js',
    output: {
        name: 'green-thumb',
        file: './release/green-thumb.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};
