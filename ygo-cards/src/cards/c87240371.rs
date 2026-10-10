//! Beetrooper Descent
use super::prelude::*;

pub const ID: u32 = 87240371;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[64213018], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,64213018,SET_BEETROOPER,TYPES_TOKEN,1000,1000,3,RACE_INSECT,ATTRIBUTE_EARTH)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_BEETROOPER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(3),
        race: Some(RACE_INSECT),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,64213018,SET_BEETROOPER,TYPES_TOKEN,1000,1000,3,RACE_INSECT,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, 64213018, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_INSECT) and c:IsAttackAbove(3000)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT) && card::is_attack_above(d, c, 3000)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,64213018,SET_BEETROOPER,TYPES_TOKEN,1000,1000,3,RACE_INSECT,ATTRIBUTE_EARTH) then return end
    if !duel::is_player_can_special_summon_monster(d, tp, 64213018, token_data(), None, None, 0) {
        return;
    }
    // local token=Duel.CreateToken(tp,64213018)
    let token = duel::create_token(d, tp, 64213018).nil();
    // if Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local dg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if #dg>0 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !dg.is_empty()
        && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=dg:Select(tp,1,1,nil)
        let sg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
