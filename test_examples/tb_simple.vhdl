library ieee;
use ieee.std_logic_1164.all;

entity tb_simple is
end entity;

architecture sim of tb_simple is
  signal s_a : std_logic := '0';
  signal s_b : std_logic := '1';
  signal v_x : std_logic_vector(3 downto 0) := (others => '0');
  signal v_y : std_logic_vector(7 downto 0) := (others => '1');
begin
  stim : process
  begin
    wait for 10 ns;
    s_a <= '1';
    v_x <= "0001";
    v_y <= x"A5";

    wait for 10 ns;
    s_b <= '0';
    v_x <= "0011";
    v_y <= x"3C";

    wait for 10 ns;
    s_a <= '0';
    s_b <= '1';
    v_x <= "0101";
    v_y <= x"FF";

    wait for 10 ns;
    s_a <= '1';
    v_x <= "1001";
    v_y <= x"00";

    wait for 10 ns;
    s_b <= '0';
    v_x <= "1110";
    v_y <= x"5A";

    wait;
  end process;
end architecture;
