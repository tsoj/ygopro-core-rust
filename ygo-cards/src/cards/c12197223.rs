//! Shiina, Twin Tempests of Celestial Thunder
use super::prelude::*;

pub const ID: u32 = 12197223;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand, then apply this effect based on that card effect's type
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // The first time this card would be destroyed by battle each turn, it is not destroyed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetValue(function(e,re,r,rp) return (r&REASON_BATTLE)>0 end)
    effect::set_value_fn(d, e2, lua_fn(indes_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,re,r,rp) return (r&REASON_BATTLE)>0 end`
fn indes_value(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_BATTLE) > 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return rp==1-tp and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WIND),tp,LOCATION_MZONE,0,1,nil)
    if a.rp != 1 - tp {
        return false;
    }
    let f = aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WIND));
    duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `aux.AND(Card.IsSpellTrap,Card.IsAbleToHand)`
fn sthfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((re:IsMonsterEffect() and Duel.IsExistingMatchingCard(s.rthfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil))
    //     or (re:IsSpellTrapEffect() and Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil))) end
    if a.chk == Some(0) {
        let re = a.re.nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && ((effect::is_monster_effect(d, re)
                && duel::is_existing_matching_card(d, Some(&rthfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX))
                || (effect::is_spell_trap_effect(d, re)
                    && duel::is_existing_matching_card(d, Some(&sthfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local g=Group.CreateGroup()
        let mut g = CardSet::new();
        let re = a.re.nil();
        // if re:IsMonsterEffect() then
        if effect::is_monster_effect(d, re) {
            // g=Duel.GetMatchingGroup(s.rthfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
            g = duel::get_matching_group(d, Some(&rthfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // elseif re:IsSpellTrapEffect(e) then
        } else if effect::is_spell_trap_effect(d, re) {
            // g=Duel.GetMatchingGroup(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
            g = duel::get_matching_group(d, Some(&sthfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        }
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
