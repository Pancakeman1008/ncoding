fn main() {
    ncurses::initscr();
    ncurses::mvprintw(5, 10, "hello");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
}