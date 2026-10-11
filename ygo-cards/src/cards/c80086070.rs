//! Ursarctic Grand Chariot
use super::prelude::*;

pub const ID: u32 = 80086070;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be special summoned by its own method
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure (from extra deck)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.sprcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| sprcon(d, a.card(1)))));
    // e2:SetTarget(s.sprtg)
    effect::set_target(d, e2, Some(lua_fn(|d, a: Args| sprtg(d, a.e(0), a.p(1)))));
    // e2:SetOperation(s.sprop)
    effect::set_operation(d, e2, Some(operation(sprop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy when flipped
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate when effect targets your Ursarctic cards
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e4, CATEGORY_NEGATE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetCost(Cost.Replaceable(s.discost))
    effect::set_cost(d, e4, Some(cost::replaceable(cost_target(discost), None)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sprfilter(c)`
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and c:HasLevel()
    card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::has_level(d, c)
}
/// `s.sprfilter1(c,tp,g,sc)`
fn sprfilter1(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local lv=c:GetLevel()
    let _lv = card::get_level(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return c:IsType(TYPE_TUNER) and c:IsLevelAbove(8) and g:IsExists(s.sprfilter2,1,c,tp,c,sc)
    card::is_type(d, c, TYPE_TUNER)
        && card::is_level_above(d, c, 8)
        && group::is_exists(d, &g, &|d, cc| sprfilter2(d, cc, tp, c, sc), 1, c.into())
}
/// `s.sprfilter2(c,tp,mc,sc)`
fn sprfilter2(d: &mut Duel, c: CardId, tp: u8, mc: CardId, sc: CardId) -> bool {
    // local sg=Group.FromCards(c,mc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    // return math.abs(c:GetLevel()-mc:GetLevel())==7 and c:IsType(TYPE_SYNCHRO) and not c:IsType(TYPE_TUNER)
    //     and Duel.GetLocationCountFromEx(tp,tp,sg,sc)>0
    let lv_c = card::get_level(d, c) as i64;
    let lv_mc = card::get_level(d, mc) as i64;
    (lv_c - lv_mc).abs() == 7
        && card::is_type(d, c, TYPE_SYNCHRO)
        && !card::is_type(d, c, TYPE_TUNER)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:IsExists(s.sprfilter1,1,nil,tp,g,c)
    group::is_exists(d, &g, &|d, cc| sprfilter1(d, cc, tp, c), 1, NO_EX)
}
/// `s.sprtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprtg(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=g:Filter(s.sprfilter1,nil,tp,g,c)
    let g1 = group::filter(d, &g, &|d, cc| sprfilter1(d, cc, tp, c), NO_EX);
    // local mg1=aux.SelectUnselectGroup(g1,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #mg1>0 then
    if !mg1.is_empty() {
        // local mc=mg1:GetFirst()
        let mc = group::get_first(d, &mg1).nil();
        // local g2=g:Filter(s.sprfilter2,mc,tp,mc,c,mc:GetLevel())
        let g2 = group::filter(d, &g, &|d, cc| sprfilter2(d, cc, tp, mc, c), mc.into());
        // local mg2=aux.SelectUnselectGroup(g2,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
            .unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
    }
    // if #mg1==2 then
    if mg1.len() == 2 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.sprop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,c)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.disfilter(c,tp)`
fn disfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_ONFIELD) and c:IsControler(tp) and c:IsSetCard(SET_URSARCTIC)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_ONFIELD) && card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_URSARCTIC as u16])
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let tg = info[0].as_group().cloned();
    // return tg and tg:IsExists(s.disfilter,1,nil,tp) and Duel.IsChainNegatable(ev)
    match tg {
        Some(tg) => {
            let tp = a.tp;
            group::is_exists(d, &tg, &|d, cc| disfilter(d, cc, tp), 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8)
        }
        None => false,
    }
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,true,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, true, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,nil,1,1,true,nil,nil)
    let sg = duel::select_release_group_cost(d, a.tp, None, 1, 1, true, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateActivation(ev)
    duel::negate_activation(d, a.ev as u8);
}
