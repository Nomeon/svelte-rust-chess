<script>
    import { onMount } from "svelte";
    import { draggable } from '@neodrag/svelte';
    import { checkMove } from '../Logic.svelte';
    import init, { valid_moves, get_index, possible_moves } from 'rust';

    // To get WASM working:
    // https://www.reddit.com/r/sveltejs/comments/vzf86d/sveltekit_with_webassembly_rust/
    // https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm


    $: board = [];
    let matrix = {};

    onMount(async() => {
        await init();
        board = createBoard();
        initializeBoard();
    })

    function createBoard() {
        let board = new Array(64);
        for (let i = 0; i < board.length; i++) {
            board[i] = {
                id: i,
                row: Math.floor(i / 8),
                col: i % 8,
                piece: "",
                color: "",
                draggable: false,
            };
        }
        return board;
    }
    
    function initializeBoard() {
        board[0].piece = "rook";
        board[1].piece = "knight";
        board[2].piece = "bishop";
        board[3].piece = "queen";
        board[4].piece = "king";
        board[5].piece = "bishop";
        board[6].piece = "knight";
        board[7].piece = "rook";
        for (let i = 8; i < 16; i++) {
            board[i].piece = "pawn";
        }

        board[56].piece = "rook";
        board[57].piece = "knight";
        board[58].piece = "bishop";
        board[59].piece = "queen";
        board[60].piece = "king";
        board[61].piece = "bishop";
        board[62].piece = "knight";
        board[63].piece = "rook";
        for (let i = 55; i > 47; i--) {
            board[i].piece = "pawn";
        }

        for (let h = 0; h < 16; h++) {
            board[h].color = "white";
            board[63-h].color = "black";
        }

        board.forEach(({row, col}) => {
            if (matrix[row]) matrix[row].push(col)
            else matrix[row] = [col]
        })
    }

    function pickPiece(e, square) {
        // const pos_moves = possible_moves(board, square);
        console.log(e)
    }

    function dropPiece(e, square) {
        let y = e.detail.offsetY;
        let x = e.detail.offsetX;

        if (!((y <= 25 && y >= 0) || (y >= -25) && (y <= 0)) || !((x <= 25 && x >= 0) || (x >= -25) && (x <= 0))) {
            let newXY = [Math.round(x/50), Math.round(y/50)];
            newXY = [newXY[0] + square.col, newXY[1] + square.row];
            if (checkMove(board, square, newXY)) {
                let resp = valid_moves(board, square, newXY[0], newXY[1]);
                console.log(resp);

                let index = get_index(newXY[0], newXY[1]);
                board[index].piece = square.piece;
                board[index].color = square.color;
                square.piece = "";
                square.color = "";
                return;
            }
        } 
        e.detail.rootNode.style.transform = "translate(0px, 0px)";
    }


</script>
<table class='board'>
    <tbody>
        {#each Object.keys(matrix) as row, i}
        <tr>
            {#each matrix[row] as k, j}
            <td class="square {((board[i * 8 + j].row % 2) + board[i * 8 + j].id) % 2 === 0 ? 'white' : 'black'}">
                {#if board[i * 8 + j].piece !== ""}
                    <div use:draggable={{ bounds: 'table' }} on:neodrag:start={(e) => pickPiece(e, board[i * 8 + j])} on:neodrag:end={(e) => dropPiece(e, board[i * 8 + j])} class='piece-square'>
                        <img src={`/pieces/${board[i * 8 + j].color}_${board[i * 8 + j].piece}.png`} alt="{board[i * 8 + j].piece}" class="piece-image"/>
                    </div>
                {/if}
            </td>
            {/each}
        </tr>
        {/each}
    </tbody>
</table>

<style>
    .board {
        border-collapse: collapse;
        border: 2px solid black;
        margin: 0 auto;
    }

    .square {
        width: 50px;
        height: 50px;
        max-width: 50px;
        max-height: 50px;
        border: 1px solid black;
        text-align: center;
        vertical-align: middle;
        padding: 0;
        box-sizing: border-box;
    }

    .piece-square {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .white {
        background-color: white;
    }

    .black {
        background-color: black;
        color: white;
    }

    .piece-image {
        max-width: 35px;
        max-height: 35px;
    }
</style>