library ieee;
use ieee.std_logic_1164.all;

entity tb_cb_toggle is
end entity;

architecture sim of tb_cb_toggle is
  signal s_watch : std_logic := '0';
begin
  stim : process
  begin
    wait for 5 ns;
    s_watch <= '1';

    wait for 5 ns;
    s_watch <= '0';

    wait for 5 ns;
    s_watch <= '1';

    wait for 5 ns;
    s_watch <= '0';

    wait for 5 ns;
    s_watch <= '1';

    wait for 5 ns;
    s_watch <= '0';

    wait for 10 ns;
    wait;
  end process;
end architecture;
