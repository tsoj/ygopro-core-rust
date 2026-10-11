//! Powerhold the Moving Battery
use super::prelude::*;

pub const ID: u32 = 35100834;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GADGET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon this card as a monster, then equip 1 "Gadget" monster from hand or deck to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2000,4,RACE_MACHINE,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2000,4,RACE_MACHINE,ATTRIBUTE_EARTH)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_EFFECT),
            attack: Some(0),
            defense: Some(2000),
            level: Some(4),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        None,
        None,
        0,
    )
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsRace(RACE_MACHINE) and c:IsSetCard(SET_GADGET)
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_MACHINE) && card::is_set_card(d, c, &[SET_GADGET as u16])
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, tp) {
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
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // local g=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_DECK|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&eqfilter), tp, LOCATION_DECK | LOCATION_HAND, 0, NO_EX);
    // if Duel.SpecialSummonComplete()==1 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and #g>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::special_summon_complete(d) == 1
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && !g.is_empty()
        && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e,nil,RESET_EVENT|RESETS_STANDARD_DISABLE)
        aux::add_er_equip_limit(
            d,
            c,
            None,
            lua_fn(|d, a: Args| eqval(d, a.c(0), a.c(1), a.p(2))),
            lua_fn(|d, a: Args| {
                equipop(d, a.c(0), a.e(1), a.p(2), a.c(3), false);
                Value::Nil
            }),
            e,
            None,
            Some(RESET_EVENT | RESETS_STANDARD_DISABLE),
            None,
        );
        // s.equipop(c,e,tp,tc,true)
        equipop(d, c, e, tp, tc, true);
    }
}
/// `s.eqval(ec,c,tp)`: `return ec:IsControler(tp) and ec:IsRace(RACE_MACHINE) and ec:GetLevel()==4`
fn eqval(d: &mut Duel, ec: CardId, _c: CardId, tp: u8) -> bool {
    card::is_controler(d, ec, tp) && card::is_race(d, ec, RACE_MACHINE) && card::get_level(d, ec) == 4
}
/// `s.equipop(c,e,tp,tc,chk)`: `local eff=false or chk`; the aux call passes no `chk` (nil, read as false)
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, chk: bool) {
    // Duel.Equip(tp,tc,c,true,eff)
    duel::equip(d, tp, tc, c, true, chk);
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
/// `s.idfilter(c)`
fn idfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local tc=c:GetEquipGroup():Filter(s.idfilter,nil):GetFirst()
    let eg = card::get_equip_group(d, c);
    let fg = group::filter(d, &eg, &idfilter, NO_EX);
    match group::get_first(d, &fg) {
        // if tc then return tc:GetAttack()*2 else return 0 end
        Some(tc) => card::get_attack(d, tc) as i64 * 2,
        None => 0,
    }
}
