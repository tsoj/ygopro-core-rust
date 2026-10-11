//! Vortex of Time
//! 時空の渦
use super::prelude::*;

pub const ID: u32 = 42138622;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, listed_names: &[CARD_CALL_OF_THE_HAUNTED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a Normal or Quick-Play Spell Card or effect, or a monster effect, while you control a Zombie monster and "Call of the Haunted" (except during the Damage Step): Your opponent tosses a coin and their activated effect becomes the following effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e1, CATEGORY_COIN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.changecon)
    effect::set_condition(d, e1, Some(condition(changecon)));
    // e1:SetTarget(s.changetg)
    effect::set_target(d, e1, Some(cost_target(changetg)));
    // e1:SetOperation(s.changeop)
    effect::set_operation(d, e1, Some(operation(changeop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.changecon(e,tp,eg,ep,ev,re,r,rp)`
fn changecon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // return rp==1-tp and (re:IsMonsterEffect() or (re:IsSpellEffect() and (rc:IsNormalSpell() or rc:IsQuickPlaySpell())))
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_CALL_OF_THE_HAUNTED),tp,LOCATION_ONFIELD,0,1,nil)
    a.rp == 1 - tp
        && (effect::is_monster_effect(d, re)
            || (effect::is_spell_effect(d, re) && (card::is_normal_spell(d, rc.nil()) || card::is_quick_play_spell(d, rc.nil()))))
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.changetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn changetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, 1 - tp, 1);
    true
}
/// `s.changeop(e,tp,eg,ep,ev,re,r,rp)`
fn changeop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // local coin=Duel.TossCoin(1-tp,1)
    let coin = duel::toss_coin(d, 1 - tp, 1)[0];
    // if coin==COIN_HEADS then
    if coin == COIN_HEADS as i32 {
        // Duel.ChangeChainOperation(ev,s.repop1)
        duel::change_chain_operation(d, a.ev as u8, operation(repop1));
    // elseif coin==COIN_TAILS then
    } else if coin == COIN_TAILS as i32 {
        // Duel.ChangeChainOperation(ev,s.repop2)
        duel::change_chain_operation(d, a.ev as u8, operation(repop2));
    }
}
/// `s.repop1(e,tp,eg,ep,ev,re,r,rp)`
fn repop1(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(opp,Card.IsAbleToRemove,opp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, opp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), opp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_RULE,PLAYER_NONE,opp)
        duel::remove(d, g, POS_FACEUP as u8, REASON_RULE, Some(PLAYER_NONE as u8), Some(opp));
    }
}
/// `s.repop2(e,tp,eg,ep,ev,re,r,rp)`
fn repop2(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_RULE)
        duel::remove(d, g, POS_FACEUP as u8, REASON_RULE, None, None);
    }
}
