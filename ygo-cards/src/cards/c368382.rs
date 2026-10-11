//! Dinomist Brachion
//! ダイナミスト・ブラキオン
use super::prelude::*;

pub const ID: u32 = 368382;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum attributes
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate effect that targets a "Dinomist" card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMIST) and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return e:GetHandler():GetFlagEffect(id)==0 and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     and g and g:IsExists(s.tfilter,1,e:GetHandler(),tp) and Duel.IsChainDisablable(ev)
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) == 0
        && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        && match g {
            Some(g) => group::is_exists(d, &g, &|d, cc| tfilter(d, cc, a.tp), 1, c.into()) && duel::is_chain_disablable(d, a.ev as u8),
            None => false,
        }
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SelectEffectYesNo(tp,c) then
    if duel::select_effect_yes_no(d, a.tp, c, None) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT + RESETS_STANDARD, 0, 1, 0, 0);
        // if Duel.NegateEffect(ev) then
        if duel::negate_effect(d, a.ev as u8) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(c,REASON_EFFECT)
            duel::destroy(d, c, REASON_EFFECT, None, None);
        }
    }
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return false end
    if g.is_empty() {
        return false;
    }
    // local tg=g:GetMaxGroup(Card.GetAttack)
    let tg = group::get_max_group(d, &g, &|d, cc| card::get_attack(d, cc) as i64).map(|x| x.0).unwrap_or_default();
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id),tp,LOCATION_MZONE,0,1,nil)
    //     and tg:IsExists(Card.IsControler,1,nil,1-tp)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, cc| card::is_code(d, cc, &[ID]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && group::is_exists(d, &tg, &|d, cc| card::is_controler(d, cc, 1 - tp), 1, NO_EX)
}
