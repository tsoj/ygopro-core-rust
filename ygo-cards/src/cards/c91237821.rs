//! Clown Crew Fiends
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91237821;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 3 monsters
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // Tribute Summoned monsters you control gain 1000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsTributeSummoned))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_tribute_summoned(d, c))));
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Tributed: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Devils" once per turn)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsSpell() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsXyzMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_xyz_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None) && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOEXTRA)
        effect::set_category(d, a.e, CATEGORY_TOEXTRA);
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,PLAYER_ALL,LOCATION_MZONE|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, PLAYER_ALL as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Return all Xyz Monsters on the field and in the GYs to the Extra Deck
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsXyzMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_xyz_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            NO_EX,
        );
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Set 1 "Clown Crew" Spell from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, a.tp, g, None, true);
        }
    }
}
