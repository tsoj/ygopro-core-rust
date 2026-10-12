//! The Sun God's Torture Device - Viser Des Shock
use super::prelude::*;

pub const ID: u32 = 101403004;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_THE_IMMORTAL_SUN_GOD],
    listed_series: &[SET_SUN_GOD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // If a monster(s) is sent to your opponent's GY and you control a "Sun God" card, even during the Damage Step: You can Special Summon this card from your hand, and if you do, return 1 other card on the field to the hand/Extra Deck, and if you do that, inflict 800 damage to your opponent
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // During the End Phase, if "The Immortal Sun God" was sent to your GY this turn: You can add this card from your GY to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    effect::set_target(d, e3, Some(cost_target(retthtg)));
    effect::set_operation(d, e3, Some(operation(retthop)));
    card::register_effect(d, c, e3, false);
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
// s.spconfilter(c,opp)
fn spconfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsMonster() and c:IsControler(opp)
    card::is_monster(d, c) && card::get_controler(d, c) == opp
}
// s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let opp = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, opp), 1, NO_EX)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SUN_GOD),tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingMatchingCard(aux.OR(Card.IsAbleToHand,Card.IsAbleToExtra),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_SUN_GOD as u16]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_hand(d, x, None) || card::is_able_to_extra(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_ONFIELD as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,aux.OR(Card.IsAbleToHand,Card.IsAbleToExtra),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c):GetFirst()
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| card::is_able_to_hand(d, x, None) || card::is_able_to_extra(d, x)),
            tp,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            1,
            false,
            c.into(),
        )
        .nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, CardSet::from_card(sc), true);
        // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND|LOCATION_EXTRA) then
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_HAND | LOCATION_EXTRA)
        {
            // Duel.Damage(1-tp,800,REASON_EFFECT)
            duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
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
