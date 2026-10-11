//! Quantum Cat
use super::prelude::*;

pub const ID: u32 = 87772572;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,typ,0,2200,4,race,att)`
fn can_summon(d: &mut Duel, tp: u8, typ: u32, race: u64, att: u32) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData { setcodes: Some(vec![0]), type_: Some(typ), attack: Some(0), defense: Some(2200), level: Some(4), race: Some(race), attribute: Some(att) },
        None,
        None,
        0,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local rac=0
        let mut rac: u64 = 0;
        // local crac=1
        let mut crac: u64 = 1;
        // while RACE_ALL&crac~=0 do
        while RACE_ALL & crac != 0 {
            // local catt=1
            let mut catt: u32 = 1;
            // while ATTRIBUTE_ALL&catt~=0 do
            while ATTRIBUTE_ALL & catt != 0 {
                // if Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,0,2200,4,crac,catt) then
                if can_summon(d, tp, TYPE_MONSTER | TYPE_NORMAL, crac, catt) {
                    // rac=rac+crac
                    rac += crac;
                    // break
                    break;
                }
                // catt=catt<<1
                catt <<= 1;
            }
            // crac=crac<<1
            crac <<= 1;
        }
        // e:SetLabel(rac)
        effect::set_label(d, e, &[rac as i64]);
        // return rac~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        return rac != 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
    // local crac=Duel.AnnounceRace(tp,1,e:GetLabel())
    let label = effect::get_label(d, e);
    let crac = duel::announce_race(d, tp, 1, label as u64);
    // local att=0
    let mut att: u32 = 0;
    // local catt=1
    let mut catt: u32 = 1;
    // while ATTRIBUTE_ALL&catt~=0 do
    while ATTRIBUTE_ALL & catt != 0 {
        // if Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,0,2200,4,crac,catt) then
        if can_summon(d, tp, TYPE_MONSTER | TYPE_NORMAL, crac, catt) {
            // att=att+catt
            att += catt;
        }
        // catt=catt<<1
        catt <<= 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTRIBUTE as u64);
    // catt=Duel.AnnounceAttribute(tp,1,att)
    catt = duel::announce_attribute(d, tp, 1, att);
    // e:SetLabel(crac)
    effect::set_label(d, e, &[crac as i64]);
    // Duel.SetTargetParam(catt)
    duel::set_target_param(d, catt);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
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
    // local rac=e:GetLabel()
    let rac = effect::get_label(d, a.e) as u64;
    // local att=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let att = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,SET_KARAKURI,0,2200,4,rac,att) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon(d, tp, SET_KARAKURI as u32, rac, att) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL+TYPE_TRAP,att,rac,0,0,0)
    card::add_monster_attribute(d, c, TYPE_NORMAL + TYPE_TRAP, att, rac, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
