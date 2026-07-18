library ieee;
use ieee.std_logic_1164.all;

entity tb_mixed is
end entity;

architecture sim of tb_mixed is
  component mixed_dut is
    port (
      a : in std_logic;
      b : in std_logic;
      c : in std_logic;
      y : out std_logic
    );
  end component;

  signal a : std_logic := '0';
  signal b : std_logic := '0';
  signal c : std_logic := '0';
  signal y : std_logic;
begin
  dut : mixed_dut
    port map (
      a => a,
      b => b,
      c => c,
      y => y
    );

  stim : process
  begin
    wait for 5 ns;
    assert y = '0'
      report "mixed: expected (0 xor 0) xor 0 = 0"
      severity failure;

    a <= '1';
    wait for 5 ns;
    assert y = '1'
      report "mixed: expected (1 xor 0) xor 0 = 1"
      severity failure;

    b <= '1';
    wait for 5 ns;
    assert y = '0'
      report "mixed: expected (1 xor 1) xor 0 = 0"
      severity failure;

    a <= '0';
    c <= '1';
    wait for 5 ns;
    assert y = '0'
      report "mixed: expected (0 xor 1) xor 1 = 0"
      severity failure;

    wait;
  end process;
end architecture;