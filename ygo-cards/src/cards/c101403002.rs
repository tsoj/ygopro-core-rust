//! The Sun God's Destructor - Makyura
use super::prelude::*;

pub const ID: u32 = 101403002;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_THE_IMMORTAL_SUN_GOD],
    listed_series: &[SET_SUN_GOD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Normal or Special Summoned, or sent from the hand or field to the GY: You can Set 1 "Sun God" Spell/Trap from your hand or Deck, and if you Set a Trap or Quick-Play Spell, it can be activated this turn
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_SET);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(settg)));
    effect::set_operation(d, e1a, Some(operation(setop)));
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone(); e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone(); e1c:SetCode(EVENT_TO_GRAVE); e1c:SetCondition(...)
    let e1c = effect::clone(d, e1a);
    effect::set_code(d, e1c, EVENT_TO_GRAVE);
    effect::set_condition(d, e1c, Some(condition(|d, a| {
        // return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD)
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_HAND | LOCATION_ONFIELD)
    })));
    card::register_effect(d, c, e1c, false);
    // During the End Phase, if "The Immortal Sun God" was sent to your GY this turn: You can add this card from your GY to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    effect::set_target(d, e2, Some(cost_target(retthtg)));
    effect::set_operation(d, e2, Some(operation(retthop)));
    card::register_effect(d, c, e2, false);
    // Check if "The Immortal Sun God" is sent to the GY
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(...)
        effect::set_operation(d, ge1, Some(operation(ge1_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// The global check's operation (for tc in eg:Iter(): IsCode(The Immortal Sun God) -> RegisterFlagEffect)
fn ge1_op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        if card::is_code(d, tc, &[CARD_THE_IMMORTAL_SUN_GOD]) {
            let p = card::get_controler(d, tc);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
// s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUN_GOD) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_SUN_GOD as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc and Duel.SSet(tp,sc)>0 then
    if let Some(sc) = group::get_first(d, &g) {
        if duel::s_set(d, tp, CardSet::from_card(sc), None, true) > 0 {
            // local effcode=nil
            // if sc:IsQuickPlaySpell() then effcode=EFFECT_QP_ACT_IN_SET_TURN
            // elseif sc:IsTrap() then effcode=EFFECT_TRAP_ACT_IN_SET_TURN end
            let effcode = if card::is_quick_play_spell(d, sc) {
                Some(EFFECT_QP_ACT_IN_SET_TURN)
            } else if card::is_trap(d, sc) {
                Some(EFFECT_TRAP_ACT_IN_SET_TURN)
            } else {
                None
            };
            // if effcode then
            if let Some(effcode) = effcode {
                // If you Set a Trap or Quick-Play Spell, it can be activated this turn
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(aux.Stringid(id,2))
                effect::set_description(d, e1, aux::stringid(ID, 2));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(effcode)
                effect::set_code(d, e1, effcode);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    }
}
// s.retthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn retthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// s.retthop(e,tp,eg,ep,ev,re,r,rp)
fn retthop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoHand(c,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
