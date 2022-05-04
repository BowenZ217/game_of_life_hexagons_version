### start:  
git clone https://github.com/PeterZ217/game_of_life_hexagons_version.git

cargo run  

### After start:  
When prompted with 

        Which status?
        1) hexagon
        2) square

    type '1' + enter: choose hexagon version of Game of Life
    type '2' + enter: choose square version of Game of Life

When prompted with

        How to set up?
        1) Use default 
        2) Use file settings 
        3) Customize

    type '1' + enter: choose blank canvas to start
    type '2' + enter: choose customfile setting from file_example folder
    type '3' + enter: choose custom rows, custom colums, and custom cell size

If choosing '2) Use file settings'

    drag custom file from ./file_example/_____.txt to terminal and drop
    press enter

If choosing '3) Customize'

    enter desired number of rows when prompted
    enter desired number of columns when prompted
    enter desired size of cell when prompted

Special key inputs while canvas is running:

    enter: next generation  
    space: automatically play / stop   
    mouse click(left): make the current mouse position of the cell switch state (dead->alive / alive -> dead)
    up arrow: speed up iterations through generations
    down arrow: slow down iterations through generations
    s: save canvas status
    r: run random cells
