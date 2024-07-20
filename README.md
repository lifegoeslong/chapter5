# chapter5
three things are listed here <br>
1: notes of chapter 5 <br>
2: codes of learning <br>
3: a simple RPG GAME implemented by rust struct <br>

游戏规则
角色（Character）：

游戏中有两个角色：英雄（Hero）和哥布林（Goblin）。
每个角色都有四个属性：名字（name）、生命值（health）、攻击力（attack）、防御力（defense）。
物品（Item）：

游戏中有两种物品：健康药水（Health Potion）和力量之剑（Sword of Might）。
每种物品可以提升角色的某些属性。
战斗：

英雄和哥布林进行回合制战斗。
每个回合，英雄先攻击哥布林，然后如果哥布林还活着，则哥布林攻击英雄。
攻击的伤害计算公式为：damage = 攻击者的攻击力 - 防御者的防御力，如果伤害值大于零，则扣除防御者的生命值。
胜负判断：

当一方角色的生命值降至零或以下时，战斗结束。
如果英雄还活着，则英雄获胜；否则，哥布林获胜。
游戏运行过程
启动游戏：

运行代码开始游戏。
初始化角色和物品：

创建英雄和哥布林角色，并设置初始属性。
创建物品健康药水和力量之剑。
使用物品：

英雄使用健康药水提升生命值。
英雄使用力量之剑提升攻击力。
开始战斗：

进入战斗循环，英雄和哥布林交替攻击，直到一方被击败。
输出战斗结果：

根据最终战斗结果输出胜利信息。

PS: 由于rust level有限，该游戏各数值均已设定(result is same)
