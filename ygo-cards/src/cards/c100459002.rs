//! Perfect Eclosion - Perfectly Ultimate Great Moth
//! 完全なる羽化－究極完全態・グレート・モス
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 100459002;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned (from your hand) by having 6 or more Insect monsters in your GY. You can only Special Summon "Perfect Eclosion - Perfectly Ultimate Great Moth" once per turn this way
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCondition(function(e,c) ... end)
    effect::set_condition(d, e0, Some(lua_fn(sp_condition)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card is Special Summoned: You can return as many cards your opponent controls as possible to the hand, then if you have a Normal Monster in your GY, you can make this card gain 300 ATK for each Insect monster in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each time your opponent Normal or Special Summons a monster(s): All monsters your opponent currently controls lose ATK equal to their own Level x 100
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2a, aux::stringid(ID, 2));
    // e2a:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCondition(s.atkcon)
    effect::set_condition(d, e2a, Some(condition(atkcon)));
    // e2a:SetOperation(s.atkop)
    effect::set_operation(d, e2a, Some(operation(atkop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}

/// The condition of e0: `function(e,c) if c==nil then return true end ... end`
fn sp_condition(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,6,nil,RACE_INSECT)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_INSECT)), tp, LOCATION_GRAVE, 0, 6, NO_EX)
}

/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, a.tp, 0);
    true
}

/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and Duel.GetOperatedGroup():IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    //     and c:IsRelateToEffect(e) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,TYPE_NORMAL) then
    if !g.is_empty()
        && duel::send_to_hand(d, g, None, REASON_EFFECT, None) > 0
        && group::is_exists(d, &duel::get_operated_group(d), &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
    {
        // local gc=Duel.GetMatchingGroupCount(Card.IsRace,tp,LOCATION_GRAVE,0,nil,RACE_INSECT)
        let gc = duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_INSECT)), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
        // if gc>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if gc > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:UpdateAttack(gc*300)
            card::update_attack(d, c, gc * 300, None, None, None);
        }
    }
}

/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}

/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.HasLevel),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::has_level(d, c))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:UpdateAttack(tc:GetLevel()*-100,nil,c)
        let lv = card::get_level(d, tc) as i64;
        card::update_attack(d, tc, lv * -100, None, Some(c), None);
    }
}
