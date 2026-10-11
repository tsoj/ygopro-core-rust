//! Metalhold the Moving Blockade
use super::prelude::*;

pub const ID: u32 = 42237854;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| effect::get_handler(d, e) != Some(c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetCondition(s.con)
    effect::set_condition(d, e3, Some(condition(con)));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| tgtg(d, e, c))));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp,chk)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1)
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:GetLevel()==4
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::get_level(d, c) == 4
}
/// The monster data of the token-like Metalhold summon `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,0,4,RACE_MACHINE,ATTRIBUTE_EARTH)`
fn md() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(0),
        defense: Some(0),
        level: Some(4),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,0,4,RACE_MACHINE,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, md(), None, None, 0);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,ft,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), n, 0, 0);
    true
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsRelateToEffect(e)
    card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,0,4,RACE_MACHINE,ATTRIBUTE_EARTH) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID, md(), None, None, 0)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, lua_fn(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.tgfilter,nil,e)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let chain_cards = info[0].as_group().cloned().nil();
    let g = group::filter(d, &chain_cards, &|d, x| tgfilter(d, x, a.e), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e,nil,RESET_EVENT|RESETS_STANDARD_DISABLE)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equip_op), a.e, None, Some(RESET_EVENT | RESETS_STANDARD_DISABLE), None);
    // if #g<=0 or ft<=0 then return end
    if g.is_empty() || ft <= 0 {
        return;
    }
    // local tg=nil
    let tg: Group = if (ft as usize) < g.len() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // tg=g:FilterSelect(tp,s.filter,ft,ft,nil)
        group::filter_select(d, &g, tp, &filter, ft as u16, ft as u16, false, NO_EX).nil()
    } else {
        // tg=g:Clone()
        g.clone()
    };
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local tc=tg:GetFirst()
        let _ = group::get_first(d, &tg);
        // for tc in aux.Next(tg) do
        for tc in group::iter(d, &tg) {
            // s.equipop(c,e,tp,tc,true)
            equipop(d, c, a.e, tp, tc, true);
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
    }
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsControler(tp) and ec:IsRace(RACE_MACHINE) and ec:GetLevel()==4
    let ec = a.c(0);
    let tp = a.p(2);
    card::is_controler(d, ec, tp) && card::is_race(d, ec, RACE_MACHINE) && card::get_level(d, ec) == 4
}
/// `function(c,e,tp,tc) s.equipop(c,e,tp,tc) end` as the `equipop` of `aux.AddEREquipLimit`
fn equip_op(d: &mut Duel, a: Args) {
    let chk = a.v(4).truthy();
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3), chk);
}
/// `s.equipop(c,e,tp,tc,chk)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, chk: bool) {
    // local eff=false or chk
    let eff = chk;
    // Duel.Equip(tp,tc,c,false,eff)
    duel::equip(d, tp, tc, c, false, eff);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(Card.EquipByEffectLimit)
    effect::set_value_fn(d, e1, card::equip_by_effect_limit());
    // e1:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, e);
    effect::set_label_object(d, e1, lo);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // local atk=0
    let mut atk: i64 = 0;
    // local g=c:GetEquipGroup()
    let c = a.c(1);
    let g = card::get_equip_group(d, c);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:GetFlagEffect(id)~=0 and tc:GetAttack()>=0 then atk=atk+tc:GetAttack() end
        if card::get_flag_effect(d, tc, ID) != 0 {
            let v = card::get_attack(d, tc);
            if v >= 0 {
                atk += v as i64;
            }
        }
    }
    // return atk
    atk
}
/// `s.tgtg(e,c)`
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    effect::get_handler(d, e) != Some(c)
}
